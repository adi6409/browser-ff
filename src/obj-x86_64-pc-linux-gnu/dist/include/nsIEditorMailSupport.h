/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorMailSupport.idl
 */

#ifndef __gen_nsIEditorMailSupport_h__
#define __gen_nsIEditorMailSupport_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsINode; /* webidl Node */


/* starting interface:    nsIEditorMailSupport */
#define NS_IEDITORMAILSUPPORT_IID_STR "fdf23301-4a94-11d3-9ce4-9960496c41bc"

#define NS_IEDITORMAILSUPPORT_IID \
  {0xfdf23301, 0x4a94, 0x11d3, \
    { 0x9c, 0xe4, 0x99, 0x60, 0x49, 0x6c, 0x41, 0xbc }}

class NS_NO_VTABLE nsIEditorMailSupport : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITORMAILSUPPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditorMailSupport;

  /* [can_run_script] Node insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertAsCitedQuotation(const nsAString& aQuotedText, const nsAString& aCitation, bool aInsertHTML, nsINode **_retval) = 0;

  /* [can_run_script] void rewrap (in boolean aRespectNewlines); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Rewrap(bool aRespectNewlines) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditorMailSupport, NS_IEDITORMAILSUPPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITORMAILSUPPORT \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertAsCitedQuotation(const nsAString& aQuotedText, const nsAString& aCitation, bool aInsertHTML, nsINode **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Rewrap(bool aRespectNewlines) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITORMAILSUPPORT \
  MOZ_CAN_RUN_SCRIPT nsresult InsertAsCitedQuotation(const nsAString& aQuotedText, const nsAString& aCitation, bool aInsertHTML, nsINode **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult Rewrap(bool aRespectNewlines); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITORMAILSUPPORT(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertAsCitedQuotation(const nsAString& aQuotedText, const nsAString& aCitation, bool aInsertHTML, nsINode **_retval) override { return _to InsertAsCitedQuotation(aQuotedText, aCitation, aInsertHTML, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Rewrap(bool aRespectNewlines) override { return _to Rewrap(aRespectNewlines); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITORMAILSUPPORT(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertAsCitedQuotation(const nsAString& aQuotedText, const nsAString& aCitation, bool aInsertHTML, nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertAsCitedQuotation(aQuotedText, aCitation, aInsertHTML, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Rewrap(bool aRespectNewlines) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Rewrap(aRespectNewlines); } 


#endif /* __gen_nsIEditorMailSupport_h__ */
