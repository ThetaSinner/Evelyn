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

if (!global.Promise) {
    global.Promise = require('bluebird');
}

var expect = require('chai').expect;
var _ = require('lodash');

var httpHelper = require('../helpers/chai_http_request_helper.js');
var commonRequestsHelper = require('../helpers/common_requests_helper.js');
var serverErrorHelper = require('../helpers/server_error_helper.js');

var userGroupHelper = require('../helpers/spec_helpers/user_group_helper.js');
var agileProjectHelper = require('../helpers/spec_helpers/agile_project_helper.js');
var agileSprintHelper = require('../helpers/spec_helpers/agile_sprint_helper.js');
var agileHeirarchyHelper = require('../helpers/spec_helpers/agile_heirarchy_helper.js');
var agileTaskHelper = require('../helpers/spec_helpers/agile_task_helper.js');

describe('Agile: Task', function() {
    var token = null;
    var altToken = null;
    var projectId = null;

    before(function () {
        return commonRequestsHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('user');
        })
        .then(function (_token) {
            token = _token;
        })
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('alt user');
        })
        .then(function (_token) {
            altToken = _token;
        })
        .then(function() {
            return agileProjectHelper.createProject(token, 'task_project');
        })
        .then(function(response) {
            projectId = response.ProjectId;  
        });
    });

    beforeEach(function() {
        return commonRequestsHelper.chaiHttpPostPurgeDatabaseArea('agile_task');
    });

    it('Creates a task', function() {
        return agileTaskHelper.createTask(token, projectId, 'starter_ref');
    });

    describe('Lookup task', function() {
        it('Looks up a task', function() {
            var taskId = null;

            return agileTaskHelper.createTask(token, projectId, 'starter_ref')
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId = response.TaskId;
                return agileTaskHelper.lookupTask(token, projectId, response.TaskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('title_starter_ref');
                expect(task.Description).to.equal('description_starter_ref');
                expect(task.OriginalEstimate).to.equal('1h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            });
        });

        it('Looks up backlog tasks', function() {
            var taskId1 = null;
            var taskId2 = null;
            var taskId3 = null;
            var sprintId = null;

            return agileTaskHelper.createTask(token, projectId, 'starter_ref_1')
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId1 = response.TaskId;
                
                return agileTaskHelper.createTask(token, projectId, 'starter_ref_2');
            })
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId2 = response.TaskId;
                
                return agileTaskHelper.createTask(token, projectId, 'starter_ref_3');
            })
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId3 = response.TaskId;
                
                return agileSprintHelper.createSprint(token, projectId, 'my sprint');
            })
            .then(function (response) {
                expect(response.SprintId).to.be.ok;
                sprintId = response.SprintId;

                return agileHeirarchyHelper.createLink(token, projectId, 'Sprint', sprintId, 'Task', taskId2);
            })
            .then(function () {
                return agileTaskHelper.lookupBacklog(token, projectId);
            })
            .then(function (response) {
                expect(response.Tasks).to.be.an.array;
                expect(response.Tasks).to.have.lengthOf(2);

                var task1 = response.Tasks[0];
                expect(task1.TaskId).to.equal(taskId1);
                expect(task1.ProjectId).to.equal(projectId);
                expect(task1.Title).to.equal('title_starter_ref_1');
                expect(task1.Assignment).to.be.null;

                var task3 = response.Tasks[1];
                expect(task3.TaskId).to.equal(taskId3);
                expect(task3.ProjectId).to.equal(projectId);
                expect(task3.Title).to.equal('title_starter_ref_3');
                expect(task3.Assignment).to.be.null;
            });
        });
    });

    describe('Update', function () {
        it('Updates a task', function () {
            var taskId = null;

            return agileTaskHelper.createTask(token, projectId, 'starter_ref', {originalEstimate: '2h'})
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId = response.TaskId;

                return agileTaskHelper.lookupTask(token, projectId, taskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('title_starter_ref');
                expect(task.Description).to.equal('description_starter_ref');
                expect(task.OriginalEstimate).to.equal('2h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            })
            .then(function () {
                return agileTaskHelper.updateTask(token, projectId, taskId, {
                    title: 'new title',
                    description: 'new description',
                    originalEstimate: '4h'
                });
            })
            .then(function () {
                return agileTaskHelper.lookupTask(token, projectId, taskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('new title');
                expect(task.Description).to.equal('new description');
                expect(task.OriginalEstimate).to.equal('4h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            });
        });

        it('Automatically updates the last modified by user', function () {
            var taskId = null;

            return agileTaskHelper.createTask(token, projectId, 'starter_ref', {originalEstimate: '2h'})
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId = response.TaskId;

                return agileTaskHelper.lookupTask(token, projectId, taskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('title_starter_ref');
                expect(task.Description).to.equal('description_starter_ref');
                expect(task.OriginalEstimate).to.equal('2h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            })
            .then(function () {
                return agileTaskHelper.updateTask(altToken, projectId, taskId, {
                    title: 'new title',
                    description: 'new description',
                    originalEstimate: '4h'
                });
            })
            .then(function () {
                return agileTaskHelper.lookupTask(token, projectId, taskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('new title');
                expect(task.Description).to.equal('new description');
                expect(task.OriginalEstimate).to.equal('4h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('alt user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            });
        });

        it('Update asigned to', function () {
            var taskId = null;
            var assignedByUserId = null;
            var assignToUserId = null;

            return agileTaskHelper.createTask(altToken, projectId, 'starter_ref', {originalEstimate: '2h'})
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId = response.TaskId;

                return agileTaskHelper.lookupTask(altToken, projectId, taskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('title_starter_ref');
                expect(task.Description).to.equal('description_starter_ref');
                expect(task.OriginalEstimate).to.equal('2h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('alt user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            })
            .then(function () {
                return commonRequestsHelper.searchForUsers(token, 'alt');
            })
            .then(function (response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                assignToUserId = response.SearchResults[0].UserId;
                
                return agileTaskHelper.updateTask(token, projectId, taskId, {
                    assignToUserId: assignToUserId
                });
            })
            .then(function () {
                return agileTaskHelper.lookupTask(token, projectId, taskId);
            })
            .then(function (response) {
                var task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('title_starter_ref');
                expect(task.Description).to.equal('description_starter_ref');
                expect(task.OriginalEstimate).to.equal('2h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.ok;
                expect(task.Assignment.AssignedToUser).to.be.ok;
                expect(task.Assignment.AssignedToUser.UserName).to.equal('alt user');
                expect(task.Assignment.AssignedToUser.UserId).to.equal(assignToUserId);
                expect(task.Assignment.AssignedByUser).to.be.ok;
                expect(task.Assignment.AssignedByUser.UserName).to.equal('user');
            });
        });
    });
});
