// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use model::ErrorModel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateStoryRequestModel {
    pub token: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateStoryResponseModel {
    pub error: Option<ErrorModel>,
    pub story_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StoryModel {
    pub story_id: String,
    pub created_by_user_id: String,
    pub date_created: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupRequestModel {
    pub token: String,
    pub project_id: String,
    pub story_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupResponseModel {
    pub error: Option<ErrorModel>,
    pub story: Option<StoryExternalModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StoryExternalModel {
    pub story_id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub tasks: Vec<TaskExternalModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TaskExternalModel {
    pub task_id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupBacklogRequestModel {
    pub token: String,
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupBacklogResponseModel {
    pub stories: Vec<StoryExternalModel>,
    pub error: Option<ErrorModel>,
}
