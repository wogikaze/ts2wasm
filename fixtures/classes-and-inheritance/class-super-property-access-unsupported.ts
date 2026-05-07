declare var B: any
class A extends B {
    constructor() {
        super();
        super.x;
        super['x'];
    }
}
