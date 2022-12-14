use crate::error::Result;
use entity::chrono::Local;
use entity::invitation_codes::{ActiveModel, Model};
use entity::invitation_record::{InvitationRecord, Model as InvitationRecordModel};
use entity::sea_orm::{ConnectionTrait, IntoActiveModel, Set};

const INVITATION_CHARS: [char; 60] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'I', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C',
    'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
    'X', 'Y', 'Z',
];

pub fn get_invitation_code_by_user_id(user_id: i32, len: usize) -> String {
    let mut code = vec![];
    let mut sl_idx = vec![0; len];

    let mut uid = user_id as usize;

    for i in 0..len {
        sl_idx[i] = (uid % INVITATION_CHARS.len()) as usize;

        let idx = (sl_idx[i] + i * sl_idx[0]) % INVITATION_CHARS.len();

        code.push(INVITATION_CHARS[idx] as u8);

        uid = uid / INVITATION_CHARS.len();
    }

    String::from_utf8_lossy(&code).to_string()
}

pub struct InvitationCodeService<'a, C> {
    pub conn: &'a C,
}

impl<'a, C: ConnectionTrait> InvitationCodeService<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        InvitationCodeService { conn }
    }

    pub async fn find(&self, active: &ActiveModel) -> Result<Model> {
        let res = active.find(self.conn).await;

        res.map_err(|e| e.into())
    }

    pub async fn find_by_user_id(&self, user_id: i32) -> Result<Model> {
        let mut active: ActiveModel = Default::default();

        active.user_id = Set(user_id);

        let res = active.find(self.conn).await;

        res.map_err(|e| e.into())
    }

    pub async fn is_valid(&self, active: &ActiveModel) -> Result<bool> {
        let model = self.find(active).await?;
        Ok(model.status)
    }

    pub async fn update(&self, active: &ActiveModel) -> Result<Model> {
        let res = active.update(self.conn).await;

        res.map_err(|e| e.into())
    }

    pub async fn create(&self, active: &ActiveModel) -> Result<Model> {
        let res = active.create(self.conn).await;

        res.map_err(|e| e.into())
    }

    pub async fn create_by_user_id(&self, user_id: i32) -> Result<Model> {
        let invitation_code = get_invitation_code_by_user_id(user_id, 6);

        let mut active: ActiveModel = Default::default();

        active.user_id = Set(user_id);
        active.invitation_code = Set(invitation_code);
        active.status = Set(false);

        let now = Local::now();

        active.create_at = Set(now.naive_local());
        active.update_at = Set(now.naive_local());

        self.create(&active).await
    }

    pub async fn create_invitation_record(
        &self,
        user_id: i32,
        code: &str,
    ) -> Result<InvitationRecordModel> {
        let active = InvitationRecord::new(user_id, code).into_active_model();

        let res = active.create(self.conn).await?;

        Ok(res)
    }
}
