const json_fetch = url => window.fetch(url)
	.then(res => res.json())
	.then(value => [null, value])
	.catch(err => [err, null])


// clones a template element, populates fields
// with data from a data object, (using handlebars notation)
// ie. {{...}} and returns the new node
function clone_template(template, data){
	// if template is a query string instead of an Element,
	// convert it to an element by querying the DOM for that
	// string
	if (typeof template === 'string'){
		template = document.querySelector(template)
	}

	const template_string = template.innerHTML
	// look for all strings enclosed by {{ }} 
	const regex = /{{(.*?)}}/gm	
	const matches = [...template_string.matchAll(regex)]
	
	let new_string = template_string
	matches.forEach(function([string, name]){
		new_string = new_string.replace(string, data[name])	
	})
	
	const newElement = document.createElement('template')
	newElement.innerHTML = new_string.trim()
	return newElement.content.firstChild
}

const media_types = {
	"image": `<img loading="lazy" class="lightbox__image" style="object-fit: contain;" src="{{url}}"></img>`,
	"video": `<video class="lightbox__image" controls src="{{url}}"></video>`,
	"model": `<model-viewer class="lightbox__image model" src="{{url}}" autorotate camera-controls></model-viewer>`
}

async function load_media() {
	const [err, media] = await json_fetch("/media")

	if(err) {
		console.error(`error fetching media metadata: ${err}`)
		return
	}	
	
	media.media.forEach(function(value, index){
		index = index + 1
		const media = (media_types[value.media_type] || "").replace("{{url}}", value.url)
		data = {media, index, next_index: index + 1, prev_index: index - 1, ...value}

		const gallery_link = clone_template('#gallery-link-template', data)
		const target = document.querySelector(`.gallery--${data.section}`)
		console.log(target)

		if (target != null) target.appendChild(gallery_link)

		const lightbox = clone_template('#lightbox-template', data)
		document.body.appendChild(lightbox)
	})
}

window.onload = function(){
	load_media()
}
