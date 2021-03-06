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

use mongodb::error::Error as MongoDbError;
use serde_json;
use bson;
use std::error;
use std::fmt;

macro_rules! EvelynErrorDisplay {
    // Both error codes and error messages
    ($x:ident, $({$x2:ident, $y:expr, $z:expr}),+) => (
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self { $($x::$x2(_) => write!(f, $y)),* }
            }
        }

        impl error::Error for $x {
            fn description(&self) -> &str {
                match *self { $($x::$x2(_) => $z),* }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self { $($x::$x2(ref e) => Some(e)),* }
            }
        }
    );

    // Only error message, Error that refers to another error
    ($x:ident, $({$x2:ident, $y:expr}),+) => (
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self { $($x::$x2(ref e) => write!(f, $y, e)),* }
            }
        }

        impl error::Error for $x {
            fn description(&self) -> &str {
                match *self { $($x::$x2(_) => $y),* }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self { $($x::$x2(ref e) => Some(e)),* }
            }
        }
    );

    // Only error message, no references to other errors
    ($x:ident, $({$x2:ident}),+) => (
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self { $($x::$x2 => write!(f, "")),* }
            }
        }

        impl error::Error for $x {
            fn description(&self) -> &str {
                match *self { $($x::$x2 => ""),* }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self { $($x::$x2 => None),* }
            }
        }
    );
}

#[derive(Debug)]
pub enum EvelynServiceError {
    ReqestForActionWhichEvelynDoesNotKnowHowToDo(EvelynBaseError),
    EvelynTriedToHandleTheRequestButDidNotYieldAResponse(EvelynBaseError),
    ExpectedHeaderOnRequestButNoneWasFound(EvelynBaseError),
    UnsupportedHttpMethod(EvelynBaseError),

    CouldNotDecodeTheRequestPayload(serde_json::Error),
    ForeignSessionToken(EvelynBaseError),

    // Server Admin
    FailedToPurge(EvelynCoreError),
    InvalidPurgeTargetType(EvelynBaseError),

    // User
    CreateUser(EvelynCoreError),
    UserAlreadyExists(EvelynCoreError),
    LogonUser(EvelynCoreError),
    FailedToLogonUser(EvelynCoreError),
    SearchForUsers(EvelynCoreError),

    // User group
    CreateUserGroup(EvelynCoreError),
    RemoveUserGroup(EvelynCoreError),
    LookupUserGroups(EvelynCoreError),
    LookupUserGroup(EvelynCoreError),
    AddMemberToUserGroup(EvelynCoreError),
    RemoveMemberFromUserGroup(EvelynCoreError),

    // Simple Task
    FailedToCreateSimpleTask(EvelynCoreError),
    FailedToUpdateSimpleTask(EvelynCoreError),
    FailedToLookupSimpleTask(EvelynCoreError),
    FailedToRemoveSimpleTask(EvelynCoreError),

    // Todo List
    CreateTodoList(EvelynCoreError),
    AddItemToTodoList(EvelynCoreError),
    LookupTodoLists(EvelynCoreError),
    LookupTodoList(EvelynCoreError),
    UpdateTodoListItem(EvelynCoreError),

    // Agile: Project
    CreateAgileProject(EvelynCoreError),
    AddUserContributorToAgileProject(EvelynCoreError),
    AddUserGroupContributorToAgileProject(EvelynCoreError),
    LookupContributingToAgileProjects(EvelynCoreError),
    LookupBacklogAgileProjects(EvelynCoreError),
    LookupAgileProject(EvelynCoreError),

    // Agile: Task
    CreateAgileTask(EvelynCoreError),
    LookupAgileTask(EvelynCoreError),
    LookupBacklogAgileTask(EvelynCoreError),
    UpdateAgileTask(EvelynCoreError),

    // Agile: Story
    CreateAgileStory(EvelynCoreError),
    LookupAgileStory(EvelynCoreError),
    LookupBacklogAgileStories(EvelynCoreError),

    // Agile: Sprint
    CreateAgileSprint(EvelynCoreError),
    LookupActiveAgileSprints(EvelynCoreError),
    LookupBacklogAgileSprints(EvelynCoreError),

    // Agile: Heirarchy
    MakeAgileHeirarchyLink(EvelynCoreError),

    // Calendar
    AddCalendarEvent(EvelynCoreError),
}

EvelynErrorDisplay!{
    EvelynServiceError,
    // Processing layer.
    {ReqestForActionWhichEvelynDoesNotKnowHowToDo, "100001", "Request for an action which Evelyn does now know how to do"},
    {EvelynTriedToHandleTheRequestButDidNotYieldAResponse, "100002", "Evelyn tried to handle the request but hasn't managed to give anything back"},
    {ExpectedHeaderOnRequestButNoneWasFound, "100003", "Expected a header with the request but didn't find a header"},
    {UnsupportedHttpMethod, "100004", "Request uses a method which the evelyn server does not know how to handle"},

    {CouldNotDecodeTheRequestPayload, "100101", "Could not decode the JSON request payload"},
    {ForeignSessionToken, "100102", "The server has been restarted please log on again"},

    // Server admin.
    {FailedToPurge, "100103", "Failed to purge"},
    {InvalidPurgeTargetType, "100104", "Invalid purge target type"},

    // User
    {CreateUser, "100201", "Failed to create user"},
    {UserAlreadyExists, "100202", "Failed to create user a user with that name already exists"},
    {LogonUser, "100203", "Invalid logon"},
    {FailedToLogonUser, "100204", "Failed to logon user"},
    {SearchForUsers, "100205", "Failed to search for users"},

    // User group
    {CreateUserGroup, "100601", "Failed to create user group"},
    {RemoveUserGroup, "100602", "Failed to remove user group"},
    {LookupUserGroups, "100603", "Failed to lookup user groups"},
    {LookupUserGroup, "100604", "Failed to lookup user group"},
    {AddMemberToUserGroup, "100605", "Failed to add member to user group"},
    {RemoveMemberFromUserGroup, "100606", "Failed to remove member from user group"},

    // Simple Task
    {FailedToCreateSimpleTask, "100301", "Failed to create simple task"},
    {FailedToUpdateSimpleTask, "100302", "Failed to update simple task"},
    {FailedToLookupSimpleTask, "100303", "Failed to lookup simple task"},
    {FailedToRemoveSimpleTask, "100304", "Failed to remove simple task"},

    // Todo List
    {CreateTodoList, "100401", "Failed to create todo list"},
    {AddItemToTodoList, "100402", "Failed to add item to todo list"},
    {LookupTodoLists, "100403", "Failed to lookup todo lists"},
    {LookupTodoList, "100404", "Failed to lookup todo list"},
    {UpdateTodoListItem, "100405", "Failed to update todo list item"},

    // Agile: Project
    {CreateAgileProject, "1006001", "Failed to create agile project"},
    {AddUserContributorToAgileProject, "1006002", "Failed to add user contributor to agile project"},
    {AddUserGroupContributorToAgileProject, "1006003", "Failed to add user group contributor to agile project"},
    {LookupContributingToAgileProjects, "1006004", "Failed to lookup contributing to agile projects"},
    {LookupBacklogAgileProjects, "1006006", "Failed to lookup agile project backlog"},
    {LookupAgileProject, "1006005", "Failed to lookup agile project"},

    // Agile: Task
    {CreateAgileTask, "1006101", "Failed to create agile task"},
    {LookupAgileTask, "1006102", "Failed to lookup agile task"},
    {LookupBacklogAgileTask, "1006104", "Failed to lookup backlog agile task"},
    {UpdateAgileTask, "1006103", "Failed to update agile task"},

    // Agile: Story
    {CreateAgileStory, "1006401", "Failed to create agile story"},
    {LookupAgileStory, "1006402", "Failed to lookup agile story"},
    {LookupBacklogAgileStories, "1006403", "Failed to lookup backlog agile stories"},

    // Agile: Sprint
    {CreateAgileSprint, "1006201", "Failed to create agile sprint"},
    {LookupActiveAgileSprints, "1006202", "Failed to lookup active sprints"},
    {LookupBacklogAgileSprints, "1006203", "Failed to lookup active sprints"},

    // Agile: Heirarchy
    {MakeAgileHeirarchyLink, "1006301", "Failed to make agile heirarchy link"},

    // Calendar
    {AddCalendarEvent, "100501", "Failed to add calendar event"}
}

#[derive(Debug)]
pub enum EvelynCoreError {
    // Server Admin
    FailedToPurgeDatabase(EvelynDatabaseError),
    FailedToPurgeDatabaseArea(EvelynDatabaseError),

    // User
    WillNotCreateUserBecauseUserAlreadyExists(EvelynBaseError),
    CannotCheckIfUserExistsSoWillNotCreateNewUser(EvelynDatabaseError),
    FailedToCreateUser(EvelynDatabaseError),
    InvalidLogon(EvelynBaseError),
    FailedToLogonUser(EvelynDatabaseError),
    FailedToSearchForUsers(EvelynDatabaseError),

    // User group
    FailedToCreateUserGroup(EvelynDatabaseError),
    FailedToRemoveUserGroup(EvelynDatabaseError),
    FailedToLookupUserGroups(EvelynDatabaseError),
    FailedToLookupUserGroup(EvelynDatabaseError),
    FailedToAddMemberToUserGroup(EvelynDatabaseError),
    FailedToRemoveMemberFromUserGroup(EvelynDatabaseError),

    // Simple Task`
    FailedToCreateSimpleTask(EvelynDatabaseError),
    FailedToUpdateSimpleTask(EvelynDatabaseError),
    FailedToLookupSimpleTask(EvelynDatabaseError),
    FailedToRemoveSimpleTask(EvelynDatabaseError),

    // Todo List
    FailedToCreateTodoList(EvelynDatabaseError),
    FailedToAddItemToTodoList(EvelynDatabaseError),
    FailedToLookupTodoLists(EvelynDatabaseError),
    FailedToLookupTodoList(EvelynDatabaseError),
    FailedToUpdateTodoListItem(EvelynDatabaseError),

    // Agile: Project
    FailedToCreateAgileProject(EvelynDatabaseError),
    FailedToAddUserContributorToAgileProject(EvelynDatabaseError),
    FailedToAddUserGroupContributorToAgileProject(EvelynDatabaseError),
    FailedToLookupContributingToAgileProjects(EvelynDatabaseError),
    FailedToLookupAgileProject(EvelynDatabaseError),

    // Agile: Task
    FailedToCreateAgileTask(EvelynDatabaseError),
    FailedToLookupAgileTask(EvelynDatabaseError),
    FailedToLookupBacklogAgileTasks(EvelynDatabaseError),
    AgileTaskNotFound(EvelynBaseError),
    FailedToUpdateAgileTask(EvelynDatabaseError),

    // Agile: Story
    FailedToCreateAgileStory(EvelynDatabaseError),
    FailedToLookupAgileStory(EvelynDatabaseError),
    AgileStoryNotFound(EvelynBaseError),
    FailedToLookupBacklogAgileStories(EvelynDatabaseError),

    // Agile: Sprint
    FailedToCreateAgileSprint(EvelynDatabaseError),
    FailedToLookupActiveAgileSprints(EvelynDatabaseError),
    FailedToLookupBacklogAgileSprints(EvelynDatabaseError),

    // Agile: Heirarchy
    AgileHeirarcyInvalidLink(EvelynBaseError),
    FailedToMakeAgileHeirarchyLink(EvelynDatabaseError),
    FailedToLookupExistingAgileHeirarchyLinksTo(EvelynDatabaseError),
    FailedToRemoveAgileHeirarchyLink(EvelynDatabaseError),
    FailedToLookupAgileHeirarchyLinks(EvelynDatabaseError),
    FailedToLookupAgileHeirarchyLinksToType(EvelynDatabaseError),

    // Calendar
    FailedToAddCalendarEvent(EvelynDatabaseError),
}

EvelynErrorDisplay!{
    EvelynCoreError,

    // Server Admin
    {FailedToPurgeDatabase, "Failed to purge database {}"},
    {FailedToPurgeDatabaseArea, "Failed to purge database area {}"},

    // User
    {WillNotCreateUserBecauseUserAlreadyExists, "Will not create the requested user because that user already exists. {}"},
    {CannotCheckIfUserExistsSoWillNotCreateNewUser, "Cannot check if the user exists so a new user will not be ceated: {}"},
    {FailedToCreateUser, "Failed to create user: {}"},
    {InvalidLogon, "Invalid logon {}"},
    {FailedToLogonUser, "Failed to logon user: {}"},
    {FailedToSearchForUsers, "Failed to search for users: {}"},

    // User Group
    {FailedToCreateUserGroup, "Failed to create user group: {}"},
    {FailedToRemoveUserGroup, "Failed to remove user group: {}"},
    {FailedToLookupUserGroups, "Failed to lookup user groups: {}"},
    {FailedToLookupUserGroup, "Failed to lookup user group: {}"},
    {FailedToAddMemberToUserGroup, "Failed to add member to user group: {}"},
    {FailedToRemoveMemberFromUserGroup, "Failed to remove member from user group: {}"},

    // Simple Task
    {FailedToCreateSimpleTask, "Failed to create task: {}"},
    {FailedToUpdateSimpleTask, "Failed to update task: {}"},
    {FailedToLookupSimpleTask, "Failed to lookup task: {}"},
    {FailedToRemoveSimpleTask, "Failed to remove task: {}"},

    // Todo List
    {FailedToCreateTodoList, "Failed to create todo list: {}"},
    {FailedToAddItemToTodoList, "Failed to add item to todo list: {}"},
    {FailedToLookupTodoLists, "Failed to lookup todo lists: {}"},
    {FailedToLookupTodoList, "Failed to lookup todo list: {}"},
    {FailedToUpdateTodoListItem, "Failed to update todo list item: {}"},

    // Agile: Project
    {FailedToCreateAgileProject, "Failed to create agile project: {}"},
    {FailedToAddUserContributorToAgileProject, "Failed to add user contributor to agile project: {}"},
    {FailedToAddUserGroupContributorToAgileProject, "Failed to add user group contributor to agile project: {}"},
    {FailedToLookupContributingToAgileProjects, "Failed to lookup contributing to agile projects: {}"},
    {FailedToLookupAgileProject, "Failed to lookup agile project: {}"},

    // Agile: Task
    {FailedToCreateAgileTask, "Failed to create agile task: {}"},
    {FailedToLookupAgileTask, "Failed to lookup agile task: {}"},
    {FailedToLookupBacklogAgileTasks, "Failed to lookup backlog agile tasks: {}"},
    {AgileTaskNotFound, "Agile task not found: {}"},
    {FailedToUpdateAgileTask, "Failed to update agile task: {}"},

    // Agile: Story
    {FailedToCreateAgileStory, "Failed to create agile story: {}"},
    {FailedToLookupAgileStory, "Failed to lookup agile story: {}"},
    {AgileStoryNotFound, "Agile story not found: {}"},
    {FailedToLookupBacklogAgileStories, "Failed to lookup backlog agile stories: {}"},

    // Agile: Sprint
    {FailedToCreateAgileSprint, "Failed to create agile sprint: {}"},
    {FailedToLookupActiveAgileSprints, "Failed to lookup active agile sprints: {}"},
    {FailedToLookupBacklogAgileSprints, "Failed to lookup backlog agile sprints: {}"},

    // Agile: Heirarchy
    {AgileHeirarcyInvalidLink, "Invalid link: {}"},
    {FailedToMakeAgileHeirarchyLink, "Failed to make agile heirarchy link: {}"},
    {FailedToLookupExistingAgileHeirarchyLinksTo, "Failed to lookup existing links to: {}"},
    {FailedToRemoveAgileHeirarchyLink, "Failed to remove agile heirarchy link: {}"},
    {FailedToLookupAgileHeirarchyLinks, "Failed to lookup agile heirarchy links: {}"},
    {FailedToLookupAgileHeirarchyLinksToType, "Failed to lookup agile heirarchy links to type: {}"},

    // Calendar
    {FailedToAddCalendarEvent, "Failed to add calendar event: {}"}
}

#[derive(Debug)]
pub enum EvelynDatabaseError {
    SerialisationFailed(EvelynBaseError),
    BSONEncodeFailed(bson::EncoderError),
    BSONDecodeFailed(bson::DecoderError),

    // Server Admin
    PurgeDatabase(MongoDbError),
    PurgeCollection(MongoDbError),

    // User
    InsertUser(MongoDbError),
    LookupUser(MongoDbError),
    SearchForUsers(MongoDbError),

    // User group
    InsertUserGroup(MongoDbError),
    RemoveUserGroup(MongoDbError),
    LookupUserGroups(MongoDbError),
    UserGroupNotFound(EvelynBaseError),
    LookupUserGroup(MongoDbError),
    AddMemberToUserGroup(MongoDbError),
    RemoveMemberFromUserGroup(MongoDbError),

    // Simple Task
    InsertSimpleTask(MongoDbError),
    UpdateSimpleTask(MongoDbError),
    LookupSimpleTask(MongoDbError),
    RemoveSimpleTask(MongoDbError),

    // Todo List
    InsertTodoList(MongoDbError),
    AddItemToTodoList(MongoDbError),
    LookupTodoLists(MongoDbError),
    TodoListNotFound(EvelynBaseError),
    LookupTodoList(MongoDbError),
    UpdateTodoListItem(MongoDbError),

    // Agile: Project
    InsertAgileProject(MongoDbError),
    AddUserContributorToAgileProject(MongoDbError),
    AddUserGroupContributorToAgileProject(MongoDbError),
    LookupContributingToAgileProjects(MongoDbError),
    AgileProjectNotFound(EvelynBaseError),
    LookupAgileProject(MongoDbError),

    // Agile: Task 
    InsertAgileTask(MongoDbError),
    LookupAgileTaskById(MongoDbError),
    LookupBacklogAgileTasks(MongoDbError),
    UpdateAgileTask(MongoDbError),

    // Agile: Story
    InsertAgileStory(MongoDbError),
    LookupAgileStory(MongoDbError),
    LookupBacklogAgileStories(MongoDbError),

    // Agile: Sprint
    InsertAgileSprint(MongoDbError),
    LookupActiveAgileSprints(MongoDbError),
    LookupBacklogAgileSprints(MongoDbError),

    // Agile: Heirarchy
    InsertAgileHeirarchyLink(MongoDbError),
    LookupAgileHeirarchyLinkTo(MongoDbError),
    RemoveAgileHeirarchyLinksById(MongoDbError),
    LookupAgileHeirarchyLinks(MongoDbError),
    LookupAgileHeirarchyLinksToType(MongoDbError),

    // Calendar
    InsertCalendarEvent(MongoDbError),
}

EvelynErrorDisplay!{
    EvelynDatabaseError,
    // Processing
    {SerialisationFailed, "Failed to serialise data for storage. {}"},
    {BSONEncodeFailed, "Failed to serialise data for storage. {}"},
    {BSONDecodeFailed, "Failed to deserialise data for storage. {}"},

    // Server Admin
    {PurgeDatabase, "Failed to purge database {}"},
    {PurgeCollection, "Failed to purge collection {}"},

    // User
    {InsertUser, "Failed to create record for new user: {}"},
    {LookupUser, "Failed to lookup user: {}"},
    {SearchForUsers, "Failed to search for users: {}"},

    // User group
    {InsertUserGroup, "Failed to create user group: {}"},
    {RemoveUserGroup, "Failed to remove user group: {}"},
    {LookupUserGroups, "Failed to lookup user groups: {}"},
    {UserGroupNotFound, "User group not found: {}"},
    {LookupUserGroup, "Failed to lookup user group: {}"},
    {AddMemberToUserGroup, "Failed to add member to user group: {}"},
    {RemoveMemberFromUserGroup, "Failed to remove member from user group: {}"},

    // Simple Task
    {InsertSimpleTask, "Failed to create new simple task: {}"},
    {UpdateSimpleTask, "Failed to update simple task: {}"},
    {LookupSimpleTask, "Failed to lookup simple tasks: {}"},
    {RemoveSimpleTask, "Failed to remove simple task: {}"},

    // Todo List
    {InsertTodoList, "Failed to insert todo list: {}"},
    {AddItemToTodoList, "Failed to add item to todo list: {}"},
    {LookupTodoLists, "Failed to lookup todo lists: {}"},
    {TodoListNotFound, "Todo list not found {}"},
    {LookupTodoList, "Failed to lookup todo list:  {}"},
    {UpdateTodoListItem, "Failed to update todo list item:  {}"},

    // Agile: Project
    {InsertAgileProject, "Failed to insert agile project: {}"},
    {AddUserContributorToAgileProject, "Failed to add user contributor to agile project: {}"},
    {AddUserGroupContributorToAgileProject, "Failed to add user group contributor to agile project: {}"},
    {LookupContributingToAgileProjects, "Failed to lookup contributing to agile projects: {}"},
    {AgileProjectNotFound, "Agile project was not found: {}"},
    {LookupAgileProject, "Failed to lookup agile project: {}"},

    // Agile: Task
    {InsertAgileTask, "Failed to insert agile task: {}"},
    {LookupAgileTaskById, "Failed to lookup agile task by id: {}"},
    {LookupBacklogAgileTasks, "Failed to lookup backlog agile tasks: {}"},
    {UpdateAgileTask, "Failed to update agile task: {}"},

    // Agile: Story
    {InsertAgileStory, "Failed to insert agile story: {}"},
    {LookupAgileStory, "Failed to lookup agile story: {}"},
    {LookupBacklogAgileStories, "Failed to lookup backlog agile stories: {}"},

    // Agile: Sprint
    {InsertAgileSprint, "Failed to insert agile sprint: {}"},
    {LookupActiveAgileSprints, "Failed to lookup active agile sprints: {}"},
    {LookupBacklogAgileSprints, "Failed to lookup backlog agile sprints: {}"},

    // Agile: Heirarchy
    {InsertAgileHeirarchyLink, "Failed to insert agile heirarchy link: {}"},
    {LookupAgileHeirarchyLinkTo, "Failed to lookup link to: {}"},
    {RemoveAgileHeirarchyLinksById, "Failed to remove links by id: {}"},
    {LookupAgileHeirarchyLinks, "Failed to lookup agile heirarchy links: {}"},
    {LookupAgileHeirarchyLinksToType, "Failed to lookup agile heirarchy links to type: {}"},

    // Calendar
    {InsertCalendarEvent, "Failed to insert calendar event: {}"}
}

// This error is a null enum that gets passed if there is no specific error to
// pass up the chain
// Side effect of using macros to generate code
#[derive(Debug)]
pub enum EvelynBaseError {
    NothingElse,
}

EvelynErrorDisplay!(EvelynBaseError, {
    NothingElse
});
