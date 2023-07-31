const { new_db, query_db, use_ns, use_db } = require('./index.node');

class Surreal {
    constructor(db) {
        this.db = db;
    }

    static async new(endpoint) {
        let db = await new_db(endpoint);

        return new Surreal(db);
    }

    async use(opts) {
        if (opts.db !== undefined) {
            await use_db(this.db, opts.db);
        }
        if (opts.ns !== undefined) {
            await use_ns(this.db, opts.ns);
        }
    }

    async query(query) {
        return await query_db(this.db, query);
    }

}

module.exports = Surreal;