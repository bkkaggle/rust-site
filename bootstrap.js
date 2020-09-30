/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/bkkaggle_github_io_bg.wasm": function() {
/******/ 			return {
/******/ 				"./bkkaggle_github_io_bg.js": {
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createElement_d00b8e24838e36e1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_createElement_d00b8e24838e36e1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_createElementNS_8f6ff05d30034b4f": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_createElementNS_8f6ff05d30034b4f"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_createTextNode_b7dc170e5271d075": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_createTextNode_b7dc170e5271d075"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_querySelector_e0528b8b8b25e9be": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_querySelector_e0528b8b8b25e9be"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_namespaceURI_36cdaf4b00c65482": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_namespaceURI_36cdaf4b00c65482"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_removeAttribute_3ae98ff009f689b3": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_removeAttribute_3ae98ff009f689b3"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setAttribute_156f15ecfed9f628": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_setAttribute_156f15ecfed9f628"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_addEventListener_27eb43df29303d67": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_addEventListener_27eb43df29303d67"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_removeEventListener_ccf115bbfa1fb019": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_removeEventListener_ccf115bbfa1fb019"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_state_a6faa97d2b62299c": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_state_a6faa97d2b62299c"](p0i32);
/******/ 					},
/******/ 					"__wbg_pushState_e37da71e3deb8da5": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_pushState_e37da71e3deb8da5"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_replaceState_c1e57cfd0600163b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_replaceState_c1e57cfd0600163b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlButtonElement_f5c73c981d727655": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_instanceof_HtmlButtonElement_f5c73c981d727655"](p0i32);
/******/ 					},
/******/ 					"__wbg_settype_b026aad6dfe74b9d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_settype_b026aad6dfe74b9d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlInputElement_aae90057bd26cb78": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_instanceof_HtmlInputElement_aae90057bd26cb78"](p0i32);
/******/ 					},
/******/ 					"__wbg_setchecked_4c76d21b2d916833": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_setchecked_4c76d21b2d916833"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_settype_d3d5022d7fd1ef0d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_settype_d3d5022d7fd1ef0d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_value_6d2605b80cdcbc03": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_value_6d2605b80cdcbc03"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setvalue_dc3cce23da13c2e9": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_setvalue_dc3cce23da13c2e9"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlTextAreaElement_2be5c0dd95f91e2f": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_instanceof_HtmlTextAreaElement_2be5c0dd95f91e2f"](p0i32);
/******/ 					},
/******/ 					"__wbg_value_036b553531ffb781": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_value_036b553531ffb781"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setvalue_fc815a91d9a4dec3": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_setvalue_fc815a91d9a4dec3"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_pathname_e5d7dbebdeeddf83": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_pathname_e5d7dbebdeeddf83"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_search_9444c9b4f9d75474": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_search_9444c9b4f9d75474"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_hash_118e5e08a994d5a7": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_hash_118e5e08a994d5a7"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_lastChild_b7825410c31c6d62": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_lastChild_b7825410c31c6d62"](p0i32);
/******/ 					},
/******/ 					"__wbg_setnodeValue_b246d98e9aea8713": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_setnodeValue_b246d98e9aea8713"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_appendChild_8658f795c44d1316": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_appendChild_8658f795c44d1316"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_insertBefore_9eecc8d5bbe722b5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_insertBefore_9eecc8d5bbe722b5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_removeChild_be8cb6ec13799e04": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_removeChild_be8cb6ec13799e04"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_PopStateEvent_5743cfa21cfcdf6c": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_instanceof_PopStateEvent_5743cfa21cfcdf6c"](p0i32);
/******/ 					},
/******/ 					"__wbg_state_1eb59e81cf66c118": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_state_1eb59e81cf66c118"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_e8f84259147dce74": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_instanceof_Window_e8f84259147dce74"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_d3b6d86af1c5d199": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_document_d3b6d86af1c5d199"](p0i32);
/******/ 					},
/******/ 					"__wbg_location_4c98b1eeb5ceccc0": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_location_4c98b1eeb5ceccc0"](p0i32);
/******/ 					},
/******/ 					"__wbg_history_3de8d9a8b716d5e0": function(p0i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_history_3de8d9a8b716d5e0"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_e2fdfe2af14a2323": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_newnoargs_e2fdfe2af14a2323"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_e9f0ce4da840ab94": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_call_e9f0ce4da840ab94"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_is_a2bc492e20d950cf": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_is_a2bc492e20d950cf"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_8172f4fed77fdb7c": function() {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_new_8172f4fed77fdb7c"]();
/******/ 					},
/******/ 					"__wbg_set_afe54b1eeb1aa77c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_set_afe54b1eeb1aa77c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_self_179e8c2a5a4c73a3": function() {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_self_179e8c2a5a4c73a3"]();
/******/ 					},
/******/ 					"__wbg_window_492cfe63a6e41dfa": function() {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_window_492cfe63a6e41dfa"]();
/******/ 					},
/******/ 					"__wbg_globalThis_8ebfea75c2dd63ee": function() {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_globalThis_8ebfea75c2dd63ee"]();
/******/ 					},
/******/ 					"__wbg_global_62ea2619f58bf94d": function() {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbg_global_62ea2619f58bf94d"]();
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper483": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/bkkaggle_github_io_bg.js"].exports["__wbindgen_closure_wrapper483"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/bkkaggle_github_io_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/bkkaggle_github_io_bg.wasm":"f3fb89db7ac2f3db7765"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\")).catch((e) => console.error('Error importing `index.js`:', e))\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });