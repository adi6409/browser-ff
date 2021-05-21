/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorObserver.idl
 */

#ifndef __gen_nsIEditorObserver_h__
#define __gen_nsIEditorObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIEditorObserver */
#define NS_IEDITOROBSERVER_IID_STR "f3ee57a6-890c-4ce0-a584-8a84bba0292e"

#define NS_IEDITOROBSERVER_IID \
  {0xf3ee57a6, 0x890c, 0x4ce0, \
    { 0xa5, 0x84, 0x8a, 0x84, 0xbb, 0xa0, 0x29, 0x2e }}

class NS_NO_VTABLE nsIEditorObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITOROBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditorObserver;

  /* void EditAction (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EditAction(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditorObserver, NS_IEDITOROBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITOROBSERVER \
  NS_IMETHOD EditAction(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITOROBSERVER \
  nsresult EditAction(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITOROBSERVER(_to) \
  NS_IMETHOD EditAction(void) override { return _to EditAction(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITOROBSERVER(_to) \
  NS_IMETHOD EditAction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EditAction(); } 


#endif /* __gen_nsIEditorObserver_h__ */
