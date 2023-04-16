window.onload = async () => {
    const response = await fetch("templates/section.handlebars");
    const raw_template = await response.text();
    const section_template = Handlebars.compile(raw_template);
    window.section_template = section_template;

    await addSection(
        section_template,
        { isEditing: false },
    );
    await addSection(
        section_template,
        { isEditing: true },
    );
};

async function addSection(template, context) {
    var node = document.createElement("template");
    node.innerHTML = template(context);
    var main = document.getElementsByTagName("main")[0];
    main.insertBefore(node.content.firstChild, main.lastElementChild);
}

function toggleVisualization(button)
{
    button.closest("section")
        .querySelector("aside")
        .classList
        .toggle("hidden")
}

function deleteSection(button)
{
    button.closest("section").remove()
}

function editSection(button)
{
    var section = button.closest("section");
    
    var node = document.createElement("template");
    node.innerHTML = window.section_template({ isEditing: true });

    section.replaceWith(node.content.firstChild);
}
