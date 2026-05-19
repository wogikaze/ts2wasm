function checkReservationCapacity(count: number): boolean {
    return count >= 1 && count <= 4;
}

console.log(checkReservationCapacity(0));
console.log(checkReservationCapacity(1));
console.log(checkReservationCapacity(2));
console.log(checkReservationCapacity(4));
console.log(checkReservationCapacity(5));
