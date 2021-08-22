document.getElementById("user_login").innerHTML = localStorage.getItem("login");
const search = document.getElementById("search-form");
const searchTable = document.getElementById('search_table');
const search_status = document.getElementById("search-status")
const exit = document.getElementById("exit");

exit.addEventListener("click", () => window.location.href = "./index.html");
search.addEventListener("input", SearchBooks);

const tbodySearch = document.getElementById('search_table_results');
const tbodyFiles = document.getElementById("user-files__tbody");
let booksSearchData =[];
let send_button;

function CreateBooksTable(is_admin = false) {
    fetch("api/book/get", {
            method:"POST",
            headers: {"jwt": localStorage.getItem("jwt")}
        }).then((result) => {
            if (result.ok) {
                result.json().then((books) => books.forEach(({title, author, text}) => {
                    CreateBookString(title, author, text, is_admin),
                    booksSearchData.push((author.toLowerCase() + title.toLowerCase()).replaceAll(" ", "").replaceAll("(", "").replaceAll(")", ""),
                    (title.toLowerCase() + author.toLowerCase()).replaceAll(" ", "").replaceAll("(", "").replaceAll(")", ""))
                }));
            } else alert("Не получилось загрузить книги");
        })
}

function CreateFileString(title,author, text){
    tr = document.createElement('tr');
    tr.className = "file__table-row";
    tr.dataset.title = title;
    tr.dataset.author = author.substring(0, author.lastIndexOf("(") - 1);
    tr.innerHTML=`
        <td>${ author }</td>
        <td>${ title }</td>
        <td><a href="data:text/plain;charset=utf-8, ${ text }" download="${ title }.txt"><i class="fas fa-long-arrow-alt-down download" data-title="${ title }"></i></a></td>
        <td><i onclick=OpenDocument() class="far fa-file-word open" data-title="${ title }"></i></td>
    `;
    tbodyFiles.appendChild(tr);
}

function CreateBookString(title, author, text, is_admin = false) {
    tr = document.createElement('tr');
    tr.dataset.search = (author.toLowerCase() + title.toLowerCase()).replaceAll(" ", "").replaceAll("(", "").replaceAll(")", "");
    tr.dataset.searchreverse = (title.toLowerCase() + author.toLowerCase()).replaceAll(" ", "").replaceAll("(", "").replaceAll(")", "");
    tr.dataset.title = title;
    tr.dataset.author = author.substring(0, author.lastIndexOf("(") - 1);
    tr.className = "search__table-row"
    tr.innerHTML=`
        <td>${ author }</td>
        <td>${ title }</td>
        <td><a href="data:text/plain;charset=utf-8,${ text }" download="${ title }.txt"><i class="fas fa-long-arrow-alt-down download" data-title="${ title }"></i></a></td>
        <td><i onclick=OpenDocument() class="far fa-file-word open" data-title="${ title }"></i></td>`;
        if (is_admin) tr.innerHTML += `<td><i onclick=DeleteRow() class="fas fa-trash delete" data-title="${ title }"></i></td>`;
    tbodySearch.appendChild(tr);
}

const file_title = document.getElementById('document__title');
console.log(file_title);
const file_author = document.getElementById('document__author');
const file_content = document.getElementById('document__content');
const file_dialog = document.getElementById('document');
const close = document.getElementById("close");
file_dialog.addEventListener("click", (e) => {CloseDocument(e)});

function CloseDocument(e) {
    if (e.target == file_dialog || e.target == close)
        file_dialog.classList.remove("document-visible");
    }

function OpenDocument() {
    let row = event.currentTarget.parentNode.parentNode;
    let author = row.dataset.author;
    let title = row.dataset.title;
    let text = row.querySelector('a[href^="data:text/plain;charset=utf-8,"]').href.substring(30);
    file_title.innerText = title;
    file_author.innerText = author;
    file_content.innerText = text;
    file_dialog.classList.add('document-visible');
}

const stylesheet = document.getElementById('search__style');

function DeleteRow(){
    file = event.currentTarget.dataset.title;
    fetch(`api/book/rm`, {
        method:  "POST",
        headers: {
            "Content-type": "application/json",
            "jwt": localStorage.getItem("jwt")
        },
        body: JSON.stringify({
            title: file
        })
    }).then((response) => {
        if (response.ok) document.querySelector(`tr[data-title="${ file }"]`).remove();
        else alert("Ошибка удаления файла");
    })
    
}

function SearchBooks() {
    search_value = search.value.toLowerCase().replaceAll(" ", "").replaceAll("(", "").replaceAll(")", "");
    if (search_value != "") {
        stylesheet.innerHTML = `
            .search__table-row[data-search*="${search_value}"], .search__table-row[data-searchreverse*="${search_value}"] {
                display: table-row;
            }
        `;
        if (booksSearchData.find(item => item.includes(search_value)) === undefined) hideSearchTable(true);
        else showSearchTable();
    } else hideSearchTable();
}

function hideSearchTable(nothing_found = false) {
    search_status.classList.remove('show-status');
    searchTable.classList.remove('table-show')
    if (nothing_found) search_status.classList.add('show-status')
}

function showSearchTable() {
    search_status.classList.remove('show-status');
    searchTable.classList.add('table-show');
}

function CreateFilesTable(){
    fetch("api/book/get", {
            method:"POST",
            headers: {"jwt": localStorage.getItem("jwt")}
        }).then((result) => {
            if (result.ok) {
                result.json().then((books) => books.forEach(({title, author, text}) => {
                    if (author.lastIndexOf(` (${localStorage.getItem("login")})`) != -1){
                        CreateFileString(title, author, text);
                        stylesheet.innerHTML = `
                            .file__table-row{
                                display: table-row;
                            }
                        `;
                    }
                }));
            } else alert("Не получилось загрузить книги");
        })
}

function HideFilesTable(){
    const table = document.getElementById("user-files");
    const button = document.getElementById("show-button");
    table.classList.remove("table-show");
    button.innerHTML = "<button id=\"show-button__button\" class=\"button\">Показать мои файлы</button>";
    const show_button = document.getElementById("show-button__button");
    show_button.addEventListener("click", ShowFilesTable)
}

function ShowFilesTable(){
    const table = document.getElementById("user-files");
    const button = document.getElementById("show-button");
    table.classList.add("table-show");
    button.innerHTML = "<button id=\"hide-button__button\" class=\"button\">Скрыть мои файлы</button>";
    const hide_button = document.getElementById("hide-button__button");
    hide_button.addEventListener("click", HideFilesTable);
}

function ChooseFile(e){
    const choosen_file = e.currentTarget.files[0].name;
    const button = document.getElementById("input-file");
    button.innerText = choosen_file;
}

function SendNewFile(e) {
    e.preventDefault()
    const fr = new FileReader();
    let book_author = document.getElementById('book_author').value;
    const book_title = document.getElementById('book_title').value;
    fr.onload = () => {
        if ((book_author != "") && (book_title != "")) {
        book_author += ` (${ localStorage.getItem("login") })`;
        fetch("api/book/add", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "jwt": localStorage.getItem("jwt")
            },
            body: JSON.stringify({
                title: book_title,
                author: book_author,
                text: fr.result
                })
            }).then((res) => {
                if (!res.ok) alert("Не удалось загрузить книгу");
                else alert("Книга успешна добавлена!");
                document.getElementById('book_author').value = "";
                document.getElementById('book_title').value = "";
                file.value = null;
                send_button.innerText = "Загрузить .txt файл";
                CreateFileString(book_title, book_author, fr.result);
            });
        } else alert("Заполните все поля!")
    }
    fr.readAsText(file.files[0]);
    fr.onerror = () => alert("Заполните все поля!");
}

CreateBooksTable(localStorage.getItem("role") === "admin");
if (localStorage.getItem("role") == "teacher"){
    CreateFilesTable();

    const show_button = document.getElementById("show-button");
    const upload_form = document.getElementById("upload-form");
    show_button.innerHTML = "<button id=\"show-button__button\" class=\"button\">Показать мои файлы</button>";
    const button = document.getElementById("show-button__button");
    button.addEventListener("click", ShowFilesTable);

    upload_form.innerHTML = `<form class="main__form__upload">
                <div class="main__form__upload-group">
                    <input class="main__form__upload-input" placeholder=" " id="book_author">
                    <label class="main__form__upload-label">Ф.И.О. всех авторов (полностью)</label>
                </div>
                <div class="main__form__upload-group">
                    <input class="main__form__upload-input" placeholder=" " id="book_title">
                    <label class="main__form__upload-label">Название</label>
                </div>
                <div class="upload-buttons">
                    <input id="file" class="file" type="file" accept=".txt">
                    <label id="input-file" for="file" class="label-button">Загрузить .txt файл</label>
                    <button class="button">Отправить</button>
                </div>
            </form>`;
    send_button = document.getElementById("input-file");
    const file = document.getElementById("file");
    file.addEventListener("change", ChooseFile)
    const bookForm = document.querySelector(".main__form__upload");
    bookForm.addEventListener("submit", (e) => {SendNewFile(e)})
}
else if (localStorage.getItem("role") == "admin"){
    const show_button = document.createElement("div");
    show_button.classList.add("show-button");
    show_button.innerHTML = "<button id=\"admin-button\" class=\"admin-button\">Перейти к панели администратора</button>";
    document.getElementById("main").appendChild(show_button);
    const admin_button = document.getElementById("admin-button");
    admin_button.addEventListener("click", () => {window.location.href = "./admin.html"});
}