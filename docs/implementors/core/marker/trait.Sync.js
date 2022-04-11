(function() {var implementors = {};
implementors["kvs"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"kvs/enum.KVSError.html\" title=\"enum kvs::KVSError\">KVSError</a>","synthetic":true,"types":["kvs::error::KVSError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"kvs/struct.KvStore.html\" title=\"struct kvs::KvStore\">KvStore</a>","synthetic":true,"types":["kvs::kv_store::KvStore"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"kvs/struct.SledStore.html\" title=\"struct kvs::SledStore\">SledStore</a>","synthetic":true,"types":["kvs::sled_store::SledStore"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"kvs/struct.KVSClient.html\" title=\"struct kvs::KVSClient\">KVSClient</a>","synthetic":true,"types":["kvs::tcp::client::KVSClient"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"kvs/enum.DBCommands.html\" title=\"enum kvs::DBCommands\">DBCommands</a>","synthetic":true,"types":["kvs::tcp::protocol::DBCommands"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"kvs/enum.ServerResponse.html\" title=\"enum kvs::ServerResponse\">ServerResponse</a>","synthetic":true,"types":["kvs::tcp::protocol::ServerResponse"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"kvs/struct.KvsServer.html\" title=\"struct kvs::KvsServer\">KvsServer</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["kvs::tcp::server::KvsServer"]}];
implementors["kvs_client"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"kvs_client/struct.Cli.html\" title=\"struct kvs_client::Cli\">Cli</a>","synthetic":true,"types":["kvs_client::Cli"]}];
implementors["kvs_server"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"kvs_server/struct.Cli.html\" title=\"struct kvs_server::Cli\">Cli</a>","synthetic":true,"types":["kvs_server::Cli"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()