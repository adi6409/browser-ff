//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/storage/nsIDOMStorageManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMStorageManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* Storage precacheStorage (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal); */
                    Method {
                        name: "PrecacheStorage",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aStoragePrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Storage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, in AString aDocumentURI, [optional] in bool aPrivate); */
                    Method {
                        name: "CreateStorage",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aStoragePrincipal", ty: "*const nsIPrincipal" }, Param { name: "aDocumentURI", ty: "*const ::nsstring::nsAString" }, Param { name: "aPrivate", ty: "bool" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Storage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, [optional] in bool aPrivate); */
                    Method {
                        name: "GetStorage",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aStoragePrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPrivate", ty: "bool" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cloneStorage (in Storage aStorageToCloneFrom); */
                    Method {
                        name: "CloneStorage",
                        params: &[Param { name: "aStorageToCloneFrom", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool checkStorage (in nsIPrincipal aPrincipal, in Storage aStorage); */
                    Method {
                        name: "CheckStorage",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aStorage", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDOMSessionStorageManager",
            base: Some("nsIDOMStorageManager"),
            methods: Err("Rust only supports [ref] / [ptr] native types"),
        },

        ]; D}

