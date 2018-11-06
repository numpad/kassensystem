
var delete_target = -1;

function deleteUsers() {
	if (delete_target < 0) { return; }
	
	alert("Lösche Benutzer " + delete_target);
	post("/api/delete_user", {delete_id: delete_target});
}

function addDeleteTarget(e) {
	if (delete_target == e.innerText) {
		document.getElementById("user_id" + delete_target).removeAttribute("style");
		delete_target = -1;
		return;
	} else if (delete_target >= 0) {
		document.getElementById("user_id" + delete_target).removeAttribute("style");
	}

	e.parentElement.setAttribute("style", "text-decoration: line-through; color: #d23;");
	delete_target = e.innerText;
}

 /**
 * sends a request to the specified url from a form. this will change the window location.
 * @param {string} path the path to send the post request to
 * @param {object} params the paramiters to add to the url
 * @param {string} [method=post] the method to use on the form
 */
function post(path, params, method) {
	method = method || "post"; // Set method to post by default if not specified.

	// The rest of this code assumes you are not using a library.
	// It can be made less wordy if you use one.
	var form = document.createElement("form");
	form.setAttribute("method", method);
	form.setAttribute("action", path);

	for(var key in params) {
		if(params.hasOwnProperty(key)) {
			var hiddenField = document.createElement("input");
			hiddenField.setAttribute("type", "hidden");
			hiddenField.setAttribute("name", key);
			hiddenField.setAttribute("value", params[key]);

			form.appendChild(hiddenField);
		}
	}

	document.body.appendChild(form);
	form.submit();
}

/**
 * Formatiere Cents in lesbaren Betrag
 * z.B.: 250 -> 2.50€
 *
 * @param cents Eingabestring Cents.
 * @return Formatierter Ausgabestring.
 */
function formatCents(cents) {
	var amount = (Number(cents) / 100).toFixed(2);
	if (amount < 0.0) {
		return "<b>- " + amount.substr(1) + "€</b>"
	}
	return amount + "€";
}

function preventFaultyInput(e) {
	var name = document.getElementById("input_name").value;
	var balance_cents = document.getElementById("input_balance").value;

	if (name.trim().length <= 1 || balance_cents.trim().length < 1) {
		e.preventDefault();
		alert('Keine valide Eingabe!');
	}

}

document.addEventListener("DOMContentLoaded", function () {
	let balances = document.getElementsByClassName("in-balance-column");
	
	for (var i = 0; i < balances.length; ++i) {
		balances[i].innerHTML = formatCents(balances[i].innerText);
	}
});
