const {execSync} = require("child_process");

const a = [
	"./2b572b4362273f582308be6429522e4c.jpg",
	"./5ed508735dec1419e35036eadb7dff68.png",
	"./a.jpg",
	"./b.jpg",
	"./background.jpeg",
	"./beloved.png",
	"./c.jpg",
	"./cum.jpg",
	"./freeps2.jpeg",
	"./gradient.png",
	"./happy.png",
	"./icon.png",
	"./jumpy.png",
	"./jumpy2.png",
	"./jumpy2_mfe.png",
	"./jumpy_mfe.png",
	"./loveyou.jpg",
	"./the.png",
];

for (const x of a) {
	let out = x.split(".")[1];
	console.log(x);
	execSync(
		`magick ${x} -quality 88 -define webp:use-sharp-yuv=true -define webp:image-hint=picture -define webp:method=6 -define webp:thread-level=1 -define webp:auto-filter=true -define webp:alpha-filtering=2 -define webp:alpha-compression=1 new/${out}.webp`
	);
}
