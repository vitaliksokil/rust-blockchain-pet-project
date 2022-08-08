'use strict';
module.exports = function(app) {
    let authController = require('../controllers/authController');

    app.route('/api/register')
        .post(authController.register);


};
