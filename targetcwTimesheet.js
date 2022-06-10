function fill() {
    const days = 5;
    
    let time_in = '';
    let time_out = '';
    const lunch_out = '12:30 PM';
    const lunch_in = '12:45 PM';
    for (var i = 1; i <= days; ++i) {
        if (i % 2 == 0) {
            time_in = '10:00 AM';
            time_out = '6:15 PM';
        } else {
            time_in = '9:30 AM';
            time_out = '5:45 PM';
        }

        let elem = document.getElementsByName(`Day${i}IN`)[0];
        elem.value = time_in;

        elem = document.getElementsByName(`Day${i}LunchOUT`)[0];
        elem.value = lunch_out;

        elem = document.getElementsByName(`Day${i}LunchIN`)[0];
        elem.value = lunch_in;

        elem = document.getElementsByName(`Day${i}OUT`)[0];
        elem.value = time_out;
    }
}


function clear() {
    const days = 5;
    for (var i = 1; i <= days; ++i) {
        let elem = document.getElementsByName(`Day${i}IN`)[0];
        elem.value = '';

        elem = document.getElementsByName(`Day${i}LunchOUT`)[0];
        elem.value = '';

        elem = document.getElementsByName(`Day${i}LunchIN`)[0];
        elem.value = '';

        elem = document.getElementsByName(`Day${i}OUT`)[0];
        elem.value = '';
    }
}

// function fill() {
//     const days = 5;
//     let time_in = 0;
//     let time_out = 0;
//     const lunch_out = 1230;
//     const lunch_in = 1300;
    
//     for (var i = 1; i <= days; ++i) {
//         if (i % 2 == 0) {
//             time_in = 900;
//         } else {
//             time_in = 1000;
//         }
//         time_out = time_in + (lunch_out - lunch_in) + 800;

//         let elem = document.getElementsByName(`Day${i}IN`)[0];
//         elem.value = time_in;

//         elem = document.getElementsByName(`Day${i}LunchOUT`)[0];
//         elem.value = lunch_out;

//         elem = document.getElementsByName(`Day${i}LunchIN`)[0];
//         elem.value = lunch_in;

//         elem = document.getElementsByName(`Day${i}OUT`)[0];
//         elem.value = time_out;
//     }
// }