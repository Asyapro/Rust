const sign_up_form = document.getElementById("sign_up_form");
const sign_in_form = document.getElementById("sign_in_form");
const user_login_input = document.getElementById("user_login");
const new_user_login_input = document.getElementById("user_login");

sign_up_form.addEventListener("submit", (e) => {
  e.preventDefault();
  if (new_user_login.value == "" || document.getElementById("new_user_password").value == "")
    alert("Заполните все поля!");
  else {
    CreateUser(document.getElementById("new_user_login").value,
      document.getElementById("new_user_password").value,
      "user");
  }
});

sign_in_form.addEventListener("submit", (e) => {
  e.preventDefault();
  AuthUser(document.getElementById("user_login").value,
      document.getElementById("user_password").value, user_login_input);
});


function CreateUser(user_mail, user_password, user_role){
  fetch("/api/user/create",{
    method: "POST",
    headers: {
      "Content-type" : "application/json"
    },
    body: JSON.stringify({
      mail: user_mail,
      password: user_password,
      role: user_role
    })
  }).then((response) => {
      AuthUser(user_mail, user_password, new_user_login_input);
  })
}

function AuthUser(user_mail, user_password, input){
  fetch("api/user/login",
  {
    method: "POST",
    headers: {
      "Content-type": "application/json"
    },
    body: JSON.stringify({
      mail: user_mail,
      password: user_password,
      role: ""
    })
  }).then((result) => {
    if (result.ok){
      result.json().then(({jwt}) => localStorage.setItem("jwt", jwt));
      localStorage.setItem("login", user_mail);
      fetch("api/auth/list", {
        method: "POST"
      }).then((res) => {
          if (res.ok){
            res.json().then((users) => {
              let user_role = users.find(item => item.login == user_mail).roles[0];
              console.log(user_role);
              localStorage.setItem("role", user_role);
              input.value = "";
              window.location.href = "./main.html"
            });
          } else alert("Ошибка загрузки списка пользователей");
      });
    } 
    else alert("Неверный логин или пароль");
  });
}
