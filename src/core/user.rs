/// Evelyn: Your personal assistant, project manager and calendar
/// Copyright (C) 2017 Gregory Jensen
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;
use std::sync::Mutex;

use model::{CreateUserModel, UserModel};
use processing::ProcessorData;

pub fn create_user(model: CreateUserModel, processor_data: Arc<ProcessorData>) {
  let user_model = UserModel{user_name: model.user_name, email_address: model.email_address, password: model.password};

  let ds = processor_data.data_store.clone();
  let mut data_store = ds.lock().unwrap();
  data_store.insert_user(&user_model);
}