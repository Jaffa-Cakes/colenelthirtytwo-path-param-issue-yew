use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
	#[at("/")]
	Root,
	#[at("/search/:query")]
	Search { query: String },
}

fn switch(route: Route) -> Html {
	match route {
		Route::Root => html!{
			<Link<Route> to={Route::Search { query: "foo bar baz".into() }}>
				{"Do search!"}
			</Link<Route>>
		},
		Route::Search { query } => html!{
			<p>{"You searched for: "}<code>{query.clone()}</code></p>
		},
	}
}

#[function_component]
fn App() -> Html {
	html!{
		<BrowserRouter>
			<Switch<Route> render={switch} />
		</BrowserRouter>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}