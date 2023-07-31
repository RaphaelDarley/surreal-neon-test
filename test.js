// const { new_db, query, use_ns, use_db } = require('.');
// import SurrealDB from "index.js";
const Surreal = require("./index.js")

async function run() {
    const db = await Surreal.new("memory");

    await db.use({ "ns": "test", "db": "test" });
    // console.log(db);

    // await use_ns(db, "test");
    // await use_db(db, "test");

    let reply = await db.query("UPDATE |test:100|");
    console.log(reply);


    // let select = await query(db, "select * from test limit 1");
    // console.log(select);
}
run();