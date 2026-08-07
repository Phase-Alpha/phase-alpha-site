// Glue between Leptos and Cloudflare Turnstile.
//
// Turnstile's implicit rendering only scans the DOM once, when api.js loads,
// which does not work for a client-side routed app: arriving at the contact
// page via the router happens long after that scan. So we render explicitly
// and cope with the two possible orderings, api.js winning the race or the
// component mounting first.
(function () {
	var widgetId = null;
	var pending = null;
	var ready = false;

	// The sitekey is public and is rendered into the document head by the
	// server, which keeps it a runtime setting rather than something baked
	// into the WASM bundle at build time.
	function siteKey() {
		var meta = document.querySelector('meta[name="turnstile-sitekey"]');
		return meta ? meta.getAttribute("content") : "";
	}

	function render(options, attempt) {
		attempt = attempt || 0;

		var sitekey = siteKey();
		if (!sitekey) {
			console.warn(
				"Turnstile: TURNSTILE_SITE_KEY is not set, skipping widget render",
			);
			return;
		}

		var container = document.getElementById("turnstile-widget");
		if (!container) {
			// Leptos may not have put the container in the DOM yet. Give it a
			// few frames before giving up rather than silently never
			// rendering the widget.
			if (attempt < 10) {
				window.requestAnimationFrame(function () {
					render(options, attempt + 1);
				});
			}
			return;
		}

		// Leaving a stale widget behind would leak an iframe on every
		// re-navigation to the page.
		if (widgetId !== null) {
			try {
				window.turnstile.remove(widgetId);
			} catch (e) {
				// Already gone, nothing to clean up.
			}
			widgetId = null;
		}

		widgetId = window.turnstile.render(container, {
			sitekey: sitekey,
			action: options.action,
			// Turnstile injects a hidden input under this name into the
			// enclosing form, which is how the token reaches the server.
			"response-field-name": options.fieldName,
			theme: "auto",
			size: "flexible",
		});
	}

	// Called from Leptos once the contact form is on the page.
	window.mountTurnstile = function (action, fieldName) {
		var options = { action: action, fieldName: fieldName };
		if (ready) {
			render(options);
		} else {
			pending = options;
		}
	};

	// Named in the api.js `onload` query parameter.
	window.onTurnstileLoad = function () {
		ready = true;
		if (pending) {
			render(pending);
			pending = null;
		}
	};

	// Tokens are single use, so a form that has already been submitted needs a
	// fresh one before it can be submitted again.
	window.resetTurnstile = function () {
		if (ready && widgetId !== null) {
			try {
				window.turnstile.reset(widgetId);
			} catch (e) {
				// Widget was removed from under us; the next mount will
				// create a new one.
			}
		}
	};
})();
