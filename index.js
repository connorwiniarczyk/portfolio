const showdown = require("showdown")
const converter = new showdown.Converter()

const fs = require("fs")
const path = require("path")
const Handlebars = require("handlebars")
const template_path = path.join(__dirname, 'index.handlebars')

const express = require("express")
const app = express()
const http = require("http")

const pages = [
	{
		title: "Battery Management System",
		subtitle: "Designed for the Lafayette FSAE Team",
		image: "media/bms/pack_drawing.PNG",
		path: "pages/bms.md",
		link: "bms",
	},
	{
		title: "Cell Man",
		subtitle: "A Cell Management Board for the BMS Described Above",
		image: "media/bms/cell_man_render.png",
		path: "pages/cell-man.md",
		link: "cell-man",
	},
	{
		title: "Flip Pixel Display",
		subtitle: "A Very Loud Display",
		image: "media/flip_pixel/",
		path: "pages/flip-pixel.md",
		link: "flip-pixel",
	},
]

const render_markdown = function(pages){
	const pages_rendered = pages.map(function(page){
		const markdown_path = path.join(__dirname, page.path)
		const markdown_string = fs.readFileSync(markdown_path, "utf8")
		const html = converter.makeHtml(markdown_string)
		page.html = html
		return page
	})

	return pages_rendered
}

app.get("/", function(req, res){
	const file = fs.readFileSync(template_path, 'utf8')
	const template = Handlebars.compile(file)
	const pages_new = render_markdown(pages)
	res.send(template({pages: pages_new}))
})

const path_stylesheets = path.join(__dirname, "/stylesheets")
const path_scripts = path.join(__dirname, "/scripts")
const path_fonts = path.join(__dirname, "/fonts")


app.use("/media", express.static('/home/connor/Shared/portfolio_media'))
app.use("/scripts", express.static(path_scripts))
app.use("/stylesheets", express.static(path_stylesheets))
app.use("/fonts", express.static(path_fonts))

app.listen(80)
