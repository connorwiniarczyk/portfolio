// Tab Menus and HashLinks
const TabMenu = function(tabs) {
	this.tab_list = tabs
	
	tabs.forEach((page, index) => {
		link = page.getAttribute('data-link')
		if(link == null) return

		console.log(link)

		HashLink.on(link, () => this.switchTo(index))
	})
}

TabMenu.prototype.switchTo = function(index, scrollTo = true) {
	// remove the "active" class from all tabs in the list
 	this.tab_list.forEach(tab => tab.classList.remove("active"))
		
	// set the desired tab to active
	this.tab_list[index].classList.add("active")
		
	if(scrollTo) this.tab_list[index].scrollIntoView(true)
}

HashLink = {}
HashLink.listeners = {}

HashLink.on = function(event, callback){
	if(HashLink.listeners[event] === undefined) HashLink.listeners[event] = []
	HashLink.listeners[event].push(callback)
}

HashLink.exec = function(event, args){
	if(HashLink.listeners[event] === undefined) return
	HashLink.listeners[event].forEach(listener => listener(args))
}

HashLink.onHash = function() {
	if(!window.location.hash) return
	
	const hash_string = window.location.hash.substring(1)
	const hash_method_match = hash_string.match(/^.*?(?=(\?|$))/gi)

	if(hash_method_match == null) throw "error: link invalid"
	const hash_method = hash_method_match[0]

	const hash_arg_match = hash_string.match(/\?.+?$/ig)
	const hash_arg_string = hash_arg_match ? hash_arg_match[0] : ""
	const hash_args = Utils.ParseURLQuery(hash_arg_string)

	HashLink.exec(hash_method, hash_args)
}

// window.addEventListener("hashchange", HashLink.onHash)

const Utils = {}

Utils.DomQuery = function(query){
	const nodelist = document.querySelectorAll(query)
	return Array.prototype.slice.call(nodelist)
}

// Utils.ParseURLQuery = function(url) {
// 	const query = url.substr(1);
// 	let result = {};

// 	query.split("&").forEach(function(part) {
// 		let item = part.split("=");
// 		result[item[0]] = decodeURIComponent(item[1]);
// 	});

// 	return result;
// }

// const SlideShow = function(images){
// 	this.index = 0
// 	this.images = images

// 	this.show(0)
// }

// SlideShow.prototype.show = function(index){
// 	this.images.forEach(function(img){
// 		img.classList.remove('active')
// 	})

// 	this.images[index].classList.add('active')
// 	this.index = index
// }

// SlideShow.prototype.next = function(){
// 	const next_index = (this.index + 1) % this.images.length
// 	this.show(next_index)
// }

// // SlideShow.prototype.begin = function(duration){
// //   setTimeout()
// // }

// window.addEventListener('load', function(){
// 	const slides = new SlideShow(document.querySelectorAll('.slideshow--demo img'))
// 	document.querySelector(".slideshow--demo").addEventListener('click', () => slides.next())
// })
