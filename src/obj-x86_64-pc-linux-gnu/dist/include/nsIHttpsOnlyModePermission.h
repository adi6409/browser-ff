/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/security/nsIHttpsOnlyModePermission.idl
 */

#ifndef __gen_nsIHttpsOnlyModePermission_h__
#define __gen_nsIHttpsOnlyModePermission_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIHttpsOnlyModePermission */
#define NS_IHTTPSONLYMODEPERMISSION_IID_STR "73f4f039-d6ff-41a7-9eb3-00db57b0b7f4"

#define NS_IHTTPSONLYMODEPERMISSION_IID \
  {0x73f4f039, 0xd6ff, 0x41a7, \
    { 0x9e, 0xb3, 0x00, 0xdb, 0x57, 0xb0, 0xb7, 0xf4 }}

class NS_NO_VTABLE nsIHttpsOnlyModePermission : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPSONLYMODEPERMISSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpsOnlyModePermission;

  enum {
    LOAD_INSECURE_DEFAULT = 0U,
    LOAD_INSECURE_ALLOW = 1U,
    LOAD_INSECURE_BLOCK = 2U,
    LOAD_INSECURE_ALLOW_SESSION = 9U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpsOnlyModePermission, NS_IHTTPSONLYMODEPERMISSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPSONLYMODEPERMISSION \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPSONLYMODEPERMISSION \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPSONLYMODEPERMISSION(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPSONLYMODEPERMISSION(_to) \


#endif /* __gen_nsIHttpsOnlyModePermission_h__ */
