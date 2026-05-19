function sendReservationEmail(isConfirmed: boolean, notificationsOn: boolean, hasEmail: boolean): string {
    if (isConfirmed && notificationsOn && hasEmail) {
        return "Send";
    }
    return "Do not send";
}

console.log(sendReservationEmail(true, true, true));
console.log(sendReservationEmail(true, true, false));
console.log(sendReservationEmail(true, false, true));
console.log(sendReservationEmail(true, false, false));
console.log(sendReservationEmail(false, true, true));
console.log(sendReservationEmail(false, false, false));
