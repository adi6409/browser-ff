/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsISharingHandlerApp.idl
 */

#ifndef __gen_nsISharingHandlerApp_h__
#define __gen_nsISharingHandlerApp_h__


#ifndef __gen_nsIMIMEInfo_h__
#include "nsIMIMEInfo.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISharingHandlerApp */
#define NS_ISHARINGHANDLERAPP_IID_STR "7111f769-53ec-41fd-b314-613661d5b6ba"

#define NS_ISHARINGHANDLERAPP_IID \
  {0x7111f769, 0x53ec, 0x41fd, \
    { 0xb3, 0x14, 0x61, 0x36, 0x61, 0xd5, 0xb6, 0xba }}

class NS_NO_VTABLE nsISharingHandlerApp : public nsIHandlerApp {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISHARINGHANDLERAPP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISharingHandlerApp;

  /* void share (in AString data, [optional] in AString title); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Share(const nsAString& data, const nsAString& title) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISharingHandlerApp, NS_ISHARINGHANDLERAPP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISHARINGHANDLERAPP \
  NS_IMETHOD Share(const nsAString& data, const nsAString& title) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISHARINGHANDLERAPP \
  nsresult Share(const nsAString& data, const nsAString& title); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISHARINGHANDLERAPP(_to) \
  NS_IMETHOD Share(const nsAString& data, const nsAString& title) override { return _to Share(data, title); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISHARINGHANDLERAPP(_to) \
  NS_IMETHOD Share(const nsAString& data, const nsAString& title) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Share(data, title); } 


#endif /* __gen_nsISharingHandlerApp_h__ */
