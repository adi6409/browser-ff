//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIDomainPolicy.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDomainPolicy",
            base: Some("nsISupports"),
            methods: Err("native type mozilla::dom::DomainPolicyClone unsupported"),
        },

        Interface {
            name: "nsIDomainSet",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void add (in nsIURI aDomain); */
                    Method {
                        name: "Add",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void remove (in nsIURI aDomain); */
                    Method {
                        name: "Remove",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "Clear",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* bool contains (in nsIURI aDomain); */
                    Method {
                        name: "Contains",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool containsSuperDomain (in nsIURI aDomain); */
                    Method {
                        name: "ContainsSuperDomain",
                        params: &[Param { name: "aDomain", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

