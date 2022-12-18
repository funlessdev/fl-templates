const _ = require('lodash');

function fl_main(input) {
    return { payload: "Hello " + (input.name ? input.name : "World") + "!" }
}

module.exports.fl_main = fl_main;