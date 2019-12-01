Logo(50);

module Logo(size=50, $fn=100) {
    hole = size/2;
    cylinderHeight = size * 1.25;

    difference() {
        sphere(d=size);
        
        cylinder(d=hole, h=cylinderHeight, center=true);
        #rotate([90, 0, 0]) cylinder(d=hole, h=cylinderHeight, center=true);
        rotate([0, 90, 0]) cylinder(d=hole, h=cylinderHeight, center=true);
    }
}

echo(version=version());