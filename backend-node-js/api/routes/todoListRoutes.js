'use strict';
module.exports = function(app) {
    var todoList = require('../controllers/todoListController');

    // todoList Routes
    app.route('/test')
        .post(todoList.test);


};
