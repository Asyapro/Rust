document.getElementById("user_login").innerHTML = localStorage.getItem("login");
const table = document.getElementById("users");
const bkgrnd = document.getElementById("backgrnd");
const add = document.getElementById("add-user-button");
const add_user = document.getElementById("add-dialog");
const close = document.getElementById("close");
const add_btn = document.getElementById("add-btn");
const create_user_form = document.getElementById("create_form");
const exit = document.getElementById("exit");
exit.addEventListener("click", () => window.location.href = "./index.html");

fetch("api/user/login",
{
	method: "POST",
	headers: {
		"Content-type": "application/json"
	},
	body: JSON.stringify({
		mail: "admin",
		password: "admin",
		role: ""
	})
}).then(() => {
	fetch('api/auth/list', {method: "POST"}).then((res) => {
		if (res.ok) {
			res.json().then((user_array) => user_array.forEach((user) => createTableRow(user)));
		}
		else alert("Ошибка загрузки списка пользователей!");
	});
});

create_user_form.addEventListener("submit", (e) => {
	e.preventDefault();
	if (document.getElementById("registration__login").value == "" || document.getElementById("registration__password").value == "")
		alert("Заполните все поля!");
	else {
		CreateUser(document.getElementById("registration__login").value,
	 		document.getElementById("registration__password").value,
	 		document.getElementById("registration__role").value);
		AddUserToTable();
	}
	
})

function CreateUser(user_mail, user_password, user_role){
	fetch("/api/user/create",{
		method: "POST",
		headers: {
			"Content-type" : "application/json",
		},
		body: JSON.stringify({
			mail: user_mail,
			password: user_password,
			role: user_role
		})
	}).then((response) => {
		CloseAddDialog();
	})
}

function DeleteRow(){
	id = event.currentTarget.dataset.user;
	fetch(`api/auth/delete`, {
		method:  "POST",
		headers: {
			"Content-type": "application/json"
		},
		body: JSON.stringify({
			login: id
		})
	}).then((response) => {
		if (response.ok) document.querySelector(`tr[data-user=${ id }]`).remove();
		else alert("Ошибка удаления пользователя");
	})
	
}

function createTableRow({login, roles}){
	let tr = document.createElement("tr");
	tr.dataset.user = login;
	const role_eng = roles[0];
	let user_role;
	if (role_eng == "admin")
		user_role = "Администратор";
	else if (role_eng == "teacher")
		user_role = "Преподаватель";
	else
		user_role = "Студент";
	if (role_eng != "admin")
		tr.innerHTML = `<td>${login}</td><td>${user_role}</td><td><i onclick=DeleteRow() data-user="${ login }"class="fas fa-trash delete"></i></td>`;
	else
		tr.innerHTML = `<td>${login}</td><td>${user_role}</td><td></td>`;
	table.appendChild(tr);
}

function AddUserToTable(){
	let login = document.getElementById("registration__login").value,
	user_role = document.getElementById("registration__role").value,
	paswd = document.getElementById("registration__password").value;

	let user = {
		login: login,
		password: paswd,
		roles: [user_role]
	};
	createTableRow(user);
	document.getElementById("registration__login").value = "";
	document.querySelector("option[value=user]").selected = true;
	document.getElementById("registration__password").value = "";
	CloseAddDialog();
}

function OpenAddDialog(){
	add_user.classList.add("dialog-visible");
	bkgrnd.classList.add("backgrnd-visible");
}

function CloseAddDialog(){
	add_user.classList.remove("dialog-visible");
	bkgrnd.classList.remove("backgrnd-visible");
}

add.addEventListener("click", OpenAddDialog);
close.addEventListener("click", CloseAddDialog);
bkgrnd.addEventListener("click", CloseAddDialog);



