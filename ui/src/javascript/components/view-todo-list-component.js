evelynDesktopApp.component('viewTodoListComponent', {
    template: '@@include(cleanHtml("src/components/todo-list/view-todo-list.partial.html"))',

    bindings: {
        todoList: '<',
    },

    controller: function($scope, $state, serverBridgeService) {
        $scope.addItem = function() {
            var todoListId = $scope.$ctrl.todoList.TodoListId;

            // TODO check new item isn't empty.
            serverBridgeService.send_to_server('/todolist/additem', {
                TodoListId: todoListId,
                TodoListItem: {
                    Text: $scope.newItem,
                    IsDone: false,
                },
            }, function(response) {
                if (response.Error === null) {
                    $state.reload('dashboard.viewtodolist');
                }
                else {
                    console.log(response.Error);
                }
            });
        };
    }
});
