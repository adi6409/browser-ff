/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/parser/html/nsIParserUtils.idl
 */

#ifndef __gen_nsIParserUtils_h__
#define __gen_nsIParserUtils_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class DocumentFragment; /* webidl DocumentFragment */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIParserUtils */
#define NS_IPARSERUTILS_IID_STR "a1101145-0025-411e-8873-fdf57bf28128"

#define NS_IPARSERUTILS_IID \
  {0xa1101145, 0x0025, 0x411e, \
    { 0x88, 0x73, 0xfd, 0xf5, 0x7b, 0xf2, 0x81, 0x28 }}

class NS_NO_VTABLE nsIParserUtils : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPARSERUTILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIParserUtils;

  enum {
    SanitizerAllowComments = 1U,
    SanitizerAllowStyle = 2U,
    SanitizerCidEmbedsOnly = 4U,
    SanitizerDropNonCSSPresentation = 8U,
    SanitizerDropForms = 16U,
    SanitizerDropMedia = 32U,
    SanitizerLogRemovals = 64U,
    SanitizerRemoveOnlyConditionalCSS = 128U
  };

  /* AString sanitize (in AString src, in unsigned long flags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Sanitize(const nsAString& src, uint32_t flags, nsAString& _retval) = 0;

  /* AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertToPlainText(const nsAString& src, uint32_t flags, uint32_t wrapCol, nsAString& _retval) = 0;

  /* DocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in Element element); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseFragment(const nsAString& fragment, uint32_t flags, bool isXML, nsIURI *baseURI, mozilla::dom::Element *element, mozilla::dom::DocumentFragment **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIParserUtils, NS_IPARSERUTILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPARSERUTILS \
  NS_IMETHOD Sanitize(const nsAString& src, uint32_t flags, nsAString& _retval) override; \
  NS_IMETHOD ConvertToPlainText(const nsAString& src, uint32_t flags, uint32_t wrapCol, nsAString& _retval) override; \
  NS_IMETHOD ParseFragment(const nsAString& fragment, uint32_t flags, bool isXML, nsIURI *baseURI, mozilla::dom::Element *element, mozilla::dom::DocumentFragment **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPARSERUTILS \
  nsresult Sanitize(const nsAString& src, uint32_t flags, nsAString& _retval); \
  nsresult ConvertToPlainText(const nsAString& src, uint32_t flags, uint32_t wrapCol, nsAString& _retval); \
  nsresult ParseFragment(const nsAString& fragment, uint32_t flags, bool isXML, nsIURI *baseURI, mozilla::dom::Element *element, mozilla::dom::DocumentFragment **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPARSERUTILS(_to) \
  NS_IMETHOD Sanitize(const nsAString& src, uint32_t flags, nsAString& _retval) override { return _to Sanitize(src, flags, _retval); } \
  NS_IMETHOD ConvertToPlainText(const nsAString& src, uint32_t flags, uint32_t wrapCol, nsAString& _retval) override { return _to ConvertToPlainText(src, flags, wrapCol, _retval); } \
  NS_IMETHOD ParseFragment(const nsAString& fragment, uint32_t flags, bool isXML, nsIURI *baseURI, mozilla::dom::Element *element, mozilla::dom::DocumentFragment **_retval) override { return _to ParseFragment(fragment, flags, isXML, baseURI, element, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPARSERUTILS(_to) \
  NS_IMETHOD Sanitize(const nsAString& src, uint32_t flags, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Sanitize(src, flags, _retval); } \
  NS_IMETHOD ConvertToPlainText(const nsAString& src, uint32_t flags, uint32_t wrapCol, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertToPlainText(src, flags, wrapCol, _retval); } \
  NS_IMETHOD ParseFragment(const nsAString& fragment, uint32_t flags, bool isXML, nsIURI *baseURI, mozilla::dom::Element *element, mozilla::dom::DocumentFragment **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseFragment(fragment, flags, isXML, baseURI, element, _retval); } 

#define NS_PARSERUTILS_CONTRACTID \
    "@mozilla.org/parserutils;1"
#define NS_PARSERUTILS_CID  \
{ 0xaf7b24cb, 0x893f, 0x41bb, { 0x96, 0x1f, 0x5a, 0x69, 0x38, 0x8e, 0x27, 0xc3 } }

#endif /* __gen_nsIParserUtils_h__ */
