function setup() {
	createCanvas(windowWidth, windowHeight);
	background(33);
	let a = createA('/hello', 'hello');
	a.position(width/2, height/2);
}

function draw() {
	fill(255);
	circle(width/2, height/2, 200);
}
