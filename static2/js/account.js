"use strict";

function setupGetAccessToken() {
  var accessTokenArea = document.getElementById("token-area");
  var accessToken = document.getElementById("access-token");
  var getTokenButton = document.getElementById("get-token");

  var htmlLoginForm = document.getElementById("login-form");
  var loginForm = new Form(htmlLoginForm);

  getTokenButton.addEventListener(
    "click",
    function(event) {
      getTokenButton.style.display = "none";
      accessTokenArea.style.display = "none";
      htmlLoginForm.style.display = "block";
    },
    false
  );

  var loginPassword = loginForm.input("login-password");

  loginPassword.setClearOnInvalid(true);
  loginPassword.addValidators({
    "Password required": valueMissing,
    "Password too short. It needs to be at least 10 characters long.": tooShort
  });

  loginForm.onSubmit(function(event) {
    makeRequest(
      "POST",
      "/auth/",
      htmlLoginForm.getElementsByClassName("output")[0],
      function(data) {
        loginPassword.value = "";
        accessToken.innerHTML = data.responseJSON.token;
        htmlLoginForm.style.display = "none";
        accessTokenArea.style.display = "block";
      },
      { 40100: () => loginPassword.setError("Invalid credentials") },
      {
        Authorization:
          "Basic " + btoa(window.username + ":" + loginPassword.value)
      }
    );
  });
}

function setupEditAccount() {
  var editForm = new Form(document.getElementById("edit-form"));

  var editDisplayName = editForm.input("edit-display-name");
  var editYtChannel = editForm.input("edit-yt-channel");
  var editPassword = editForm.input("edit-password");
  var authPassword = editForm.input("auth-password");

  editForm.addValidators({
    "edit-yt-channel": {
      "Please enter a valid URL": typeMismatch
    },
    "edit-password": {
      "Password too short. It needs to be at least 10 characters long.": tooShort
    },
    "edit-password-repeat": {
      "Password too short. It needs to be at least 10 characters long.": tooShort,
      "Passwords don't match": rpp => rpp.value == editPassword.value
    },
    "auth-password": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort
    }
  });

  editForm.onSubmit(function(event) {
    var data = {};

    if (editDisplayName.value) data["display_name"] = editDisplayName.value;
    if (editYtChannel.value) data["youtube_channel"] = editYtChannel.value;
    if (editPassword.value) data["password"] = editPassword.value;

    makeRequest(
      "PATCH",
      "/auth/me/",
      editForm.errorOutput,
      () => window.location.reload(),
      {
        40100: () => authPassword.setError("Invalid credentials"),
        41200: () =>
          editForm.setError(
            "Concurrent account access was made. Please reload the page"
          ),
        41800: () =>
          editForm.setError(
            "Concurrent account access was made. Please reload the page"
          ),
        42225: message => editYtChannel.setError(message)
      },
      {
        "If-Match": window.etag,
        Authorization:
          "Basic " + btoa(window.username + ":" + authPassword.value)
      },
      data
    );
  });
}

function setupInvalidateToken() {
  var invalidateButton = document.getElementById("invalidate-token");
  var htmlInvalidateForm = document.getElementById("invalidate-form");
  var invalidateForm = new Form(htmlInvalidateForm);

  invalidateButton.addEventListener(
    "click",
    function(event) {
      invalidateButton.style.display = "none";
      htmlInvalidateForm.style.display = "block";
    },
    false
  );

  var invalidatePassword = invalidateForm.input("invalidate-auth-password");

  invalidatePassword.setClearOnInvalid(true);
  invalidateForm.addValidators({
    "invalidate-auth-password": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort
    }
  });

  invalidateForm.onSubmit(function(event) {
    makeRequest(
      "POST",
      "/auth/invalidate/",
      invalidateForm.errorOutput,
      () => window.location.reload(),
      { 40100: () => loginPassword.setError("Invalid credentials") },
      {
        Authorization:
          "Basic " + btoa(window.username + ":" + invalidatePassword.value)
      }
    );
  });
}

function setupEditUser() {}

$(document).ready(function() {
  var csrfTokenSpan = document.getElementById("chicken-salad-red-fish");
  var csrfToken = csrfTokenSpan.innerHTML;

  csrfTokenSpan.remove();

  setupGetAccessToken();
  setupEditAccount();
  setupInvalidateToken();

  var deleteUserButton = document.getElementById("delete-user");

  deleteUserButton.addEventListener(
    "click",
    function(event) {
      makeRequest(
        "DELETE",
        "/users/" + window.currentUser.id + "/",
        editForm.errorOutput,
        () => editForm.setSuccess("Successfully deleted user!"),
        {},
        {
          "X-CSRF-TOKEN": csrfToken,
          "If-Match": window.currentUser.etag
        }
      );
    },
    false
  );

  var htmlEditForm = document.getElementById("patch-permissions");
  var editForm = new Form(htmlEditForm);

  var extended = editForm.input("perm-extended");
  var list_helper = editForm.input("perm-list-helper");
  var list_mod = editForm.input("perm-list-mod");
  var list_admin = editForm.input("perm-list-admin");
  var mod = editForm.input("perm-mod");
  var admin = editForm.input("perm-admin");

  var text = document.getElementById("text");

  var userByIdForm = new Form(document.getElementById("find-id-form"));
  var userByNameForm = new Form(document.getElementById("find-name-form"));

  var userId = userByIdForm.input("find-id");
  var userName = userByNameForm.input("find-name");

  userId.addValidator(valueMissing, "User ID required");
  userName.addValidators({
    "Username required": valueMissing,
    "Username is at least 3 characters long": tooShort
  });

  function requestUserForEdit(userId, errorCodes) {
    makeRequest(
      "GET",
      "/users/" + userId + "/",
      editForm.errorOutput,
      data => {
        window.currentUser = data.responseJSON.data;
        window.currentUser.etag = data.getResponseHeader("ETag");

        if (window.currentUser.name == window.username) {
          editForm.setError(
            "This is your own account. You cannot modify your own account using this interface!"
          );
        }

        if (window.currentUser.display_name) {
          text.innerHTML =
            "<b>Username: </b>" +
            window.currentUser.name +
            " (" +
            window.currentUser.display_name +
            ")" +
            "&nbsp;&nbsp;&nbsp;&nbsp;<b>User ID: </b>" +
            window.currentUser.id;
        } else {
          text.innerHTML =
            "<b>Username: </b>" +
            window.currentUser.name +
            "&nbsp;&nbsp;&nbsp;&nbsp;<b>User ID: </b>" +
            window.currentUser.id;
        }

        let bitmask = window.currentUser.permissions;

        extended.value = (bitmask & 0x1) == 0x1;
        list_helper.value = (bitmask & 0x2) == 0x2;
        list_mod.value = (bitmask & 0x4) == 0x4;
        list_admin.value = (bitmask & 0x8) == 0x8;
        mod.value = (bitmask & 0x2000) == 0x2000;
        admin.value = (bitmask & 0x4000) == 0x4000;

        htmlEditForm.style.display = "block";
      },
      errorCodes
    );
  }

  userByIdForm.onSubmit(function(event) {
    requestUserForEdit(userId.value, {
      40401: message => userId.setError(message)
    });
  });

  userByNameForm.onSubmit(function(event) {
    makeRequest(
      "GET",
      "/users/?name=" + userName.value,
      userByNameForm.errorOutput,
      data => {
        let json = data.responseJSON;

        if (!json || json.length == 0) {
          userName.setError("No user with that name found!");
        } else {
          requestUserForEdit(json[0].id, data =>
            userByNameForm.setError(data.responseJSON.message)
          );
        }
      }
    );
  });

  editForm.onSubmit(function(event) {
    makeRequest(
      "PATCH",
      "/users/" + window.currentUser.id + "/",
      editForm.errorOutput,
      data => {
        if (data.status == 200) {
          window.currentUser = data.responseJSON.data;
          window.currentUser.etag = data.getResponseHeader("ETag");

          editForm.setSuccess("Successfully modified user!");
        } else {
          editForm.setSuccess("No changes made!");
        }
      },
      {},
      {
        "X-CSRF-TOKEN": csrfToken,
        "If-Match": window.currentUser.etag
      },
      {
        permissions:
          extended.value * 0x1 +
          list_helper.value * 0x2 +
          list_mod.value * 0x4 +
          list_admin.value * 0x8 +
          mod.value * 0x2000 +
          admin.value * 0x4000
      }
    );
  });

  var loadUsersButton = document.getElementById("load-users");
  var loadUsersError = document.getElementById("load-users-error");
  var userList = document.getElementById("user-list");
  var nextUserButton = document.getElementById("next-user");
  var prevUserButton = document.getElementById("prev-user");

  function populateUserlist(data) {
    loadUsersButton.style.display = "none";

    window.userPagination = parsePagination(data.getResponseHeader("Links"));
    var userString = "";

    for (let user of data.responseJSON) {
      userString +=
        "<li data-uid=" +
        user.id +
        "><b>" +
        user.name +
        "</b> (ID: " +
        user.id +
        ")<br><i>Display name: " +
        (user.display_name || "None") +
        "</i>";
    }

    userList.innerHTML = userString;

    for (let li of userList.getElementsByTagName("li")) {
      li.addEventListener(
        "click",
        () => requestUserForEdit(li.dataset.uid),
        false
      );
    }

    document.getElementById("hidden-user-list").style.display = "block";
  }

  loadUsersButton.addEventListener("click", () =>
    makeRequest("GET", "/users/?limit=5", loadUsersError, populateUserlist)
  );

  nextUserButton.addEventListener("click", () => {
    if (window.userPagination.next) {
      makeRequest(
        "GET",
        window.userPagination.next,
        loadUsersError,
        populateUserlist
      );
    }
  });

  prevUserButton.addEventListener("click", () => {
    if (window.userPagination.prev) {
      makeRequest(
        "GET",
        window.userPagination.prev,
        loadUsersError,
        populateUserlist
      );
    }
  });
});

function parsePagination(linkHeader) {
  var links = {};
  if (linkHeader) {
    for (var link of linkHeader.split(",")) {
      var s = link.split(";");

      links[s[1].substring(5)] = s[0].substring(8, s[0].length - 1);
    }
  }
  return links;
}

function makeRequest(
  method,
  endpoint,
  errorOutput,
  onSuccess,
  errorCodes = {},
  headers = {},
  data = {}
) {
  errorOutput.style.display = "";

  headers["Accept"] = "application/json";

  $.ajax({
    method: method,
    url: "/api/v1" + endpoint,
    contentType: "application/json",
    data: JSON.stringify(data),
    headers: headers,
    error: function(data, code, errorThrown) {
      if (!data.responseJSON) {
        errorOutput.innerHTML =
          "Server unexpectedly returned " + code + " (" + errorThrown + ")";
        errorOutput.style.display = "block";
      } else {
        var error = data.responseJSON;

        if (error.code in errorCodes) {
          errorCodes[error.code](error.message, error.data);
        } else {
          console.warn(
            "The server returned an error of code " +
              error.code +
              ", which this form is not setup to handle correctly. Handling as generic error"
          );
          errorOutput.innerHTML = error.message;
          errorOutput.style.display = "block";
        }
      }
    },
    success: function(crap, crap2, data) {
      onSuccess(data);
    }
  });
}