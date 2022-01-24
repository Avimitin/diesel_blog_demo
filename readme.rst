blog_demo
=========
Blog_demo implement the `diesel <https://github.com/diesel-rs/diesel>`_ ORM crate.

Usage
-----
First initiate the docker server::

  $ docker run --name diesel_demo --rm -p 5432:5432 -e POSTGRES_PASSWORD=DIESEL_DEMO_PWD -e POSTGRES_DB=diesel_demo -d postgres:14

Then run the cli::

  $ cargo run --bin show_posts

Roadmap
-------
See `plan <./plan.org>`_ .
