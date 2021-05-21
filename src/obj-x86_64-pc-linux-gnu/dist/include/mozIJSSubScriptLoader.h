/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/mozIJSSubScriptLoader.idl
 */

#ifndef __gen_mozIJSSubScriptLoader_h__
#define __gen_mozIJSSubScriptLoader_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIObserver; /* forward declaration */


/* starting interface:    mozIJSSubScriptLoader */
#define MOZIJSSUBSCRIPTLOADER_IID_STR "19533e7b-f321-4ef1-bc59-6e812dc2a733"

#define MOZIJSSUBSCRIPTLOADER_IID \
  {0x19533e7b, 0xf321, 0x4ef1, \
    { 0xbc, 0x59, 0x6e, 0x81, 0x2d, 0xc2, 0xa7, 0x33 }}

class NS_NO_VTABLE mozIJSSubScriptLoader : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIJSSUBSCRIPTLOADER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIJSSubScriptLoader;

  /* [implicit_jscontext] jsval loadSubScript (in AString url, [optional] in jsval obj); */
  NS_IMETHOD LoadSubScript(const nsAString& url, JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval loadSubScriptWithOptions (in AString url, in jsval options); */
  NS_IMETHOD LoadSubScriptWithOptions(const nsAString& url, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIJSSubScriptLoader, MOZIJSSUBSCRIPTLOADER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIJSSUBSCRIPTLOADER \
  NS_IMETHOD LoadSubScript(const nsAString& url, JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD LoadSubScriptWithOptions(const nsAString& url, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIJSSUBSCRIPTLOADER \
  nsresult LoadSubScript(const nsAString& url, JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult LoadSubScriptWithOptions(const nsAString& url, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIJSSUBSCRIPTLOADER(_to) \
  NS_IMETHOD LoadSubScript(const nsAString& url, JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) override { return _to LoadSubScript(url, obj, cx, _retval); } \
  NS_IMETHOD LoadSubScriptWithOptions(const nsAString& url, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) override { return _to LoadSubScriptWithOptions(url, options, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIJSSUBSCRIPTLOADER(_to) \
  NS_IMETHOD LoadSubScript(const nsAString& url, JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadSubScript(url, obj, cx, _retval); } \
  NS_IMETHOD LoadSubScriptWithOptions(const nsAString& url, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadSubScriptWithOptions(url, options, cx, _retval); } 


#endif /* __gen_mozIJSSubScriptLoader_h__ */
