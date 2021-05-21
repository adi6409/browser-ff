/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleDocument.idl
 */

#ifndef __gen_nsIAccessibleDocument_h__
#define __gen_nsIAccessibleDocument_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAccessiblePivot; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIAccessibleDocument */
#define NS_IACCESSIBLEDOCUMENT_IID_STR "5cad5f91-fcce-40e7-913e-4671701d19b4"

#define NS_IACCESSIBLEDOCUMENT_IID \
  {0x5cad5f91, 0xfcce, 0x40e7, \
    { 0x91, 0x3e, 0x46, 0x71, 0x70, 0x1d, 0x19, 0xb4 }}

class NS_NO_VTABLE nsIAccessibleDocument : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEDOCUMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleDocument;

  /* readonly attribute AString URL; */
  NS_IMETHOD GetURL(nsAString& aURL) = 0;

  /* readonly attribute AString title; */
  NS_IMETHOD GetTitle(nsAString& aTitle) = 0;

  /* readonly attribute AString mimeType; */
  NS_IMETHOD GetMimeType(nsAString& aMimeType) = 0;

  /* readonly attribute AString docType; */
  NS_IMETHOD GetDocType(nsAString& aDocType) = 0;

  /* readonly attribute Document DOMDocument; */
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) = 0;

  /* readonly attribute mozIDOMWindowProxy window; */
  NS_IMETHOD GetWindow(mozIDOMWindowProxy **aWindow) = 0;

  /* readonly attribute nsIAccessibleDocument parentDocument; */
  NS_IMETHOD GetParentDocument(nsIAccessibleDocument **aParentDocument) = 0;

  /* readonly attribute unsigned long childDocumentCount; */
  NS_IMETHOD GetChildDocumentCount(uint32_t *aChildDocumentCount) = 0;

  /* readonly attribute nsIAccessiblePivot virtualCursor; */
  NS_IMETHOD GetVirtualCursor(nsIAccessiblePivot **aVirtualCursor) = 0;

  /* nsIAccessibleDocument getChildDocumentAt (in unsigned long index); */
  NS_IMETHOD GetChildDocumentAt(uint32_t index, nsIAccessibleDocument **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleDocument, NS_IACCESSIBLEDOCUMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEDOCUMENT \
  NS_IMETHOD GetURL(nsAString& aURL) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD GetMimeType(nsAString& aMimeType) override; \
  NS_IMETHOD GetDocType(nsAString& aDocType) override; \
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) override; \
  NS_IMETHOD GetWindow(mozIDOMWindowProxy **aWindow) override; \
  NS_IMETHOD GetParentDocument(nsIAccessibleDocument **aParentDocument) override; \
  NS_IMETHOD GetChildDocumentCount(uint32_t *aChildDocumentCount) override; \
  NS_IMETHOD GetVirtualCursor(nsIAccessiblePivot **aVirtualCursor) override; \
  NS_IMETHOD GetChildDocumentAt(uint32_t index, nsIAccessibleDocument **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEDOCUMENT \
  nsresult GetURL(nsAString& aURL); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult GetMimeType(nsAString& aMimeType); \
  nsresult GetDocType(nsAString& aDocType); \
  nsresult GetDOMDocument(mozilla::dom::Document **aDOMDocument); \
  nsresult GetWindow(mozIDOMWindowProxy **aWindow); \
  nsresult GetParentDocument(nsIAccessibleDocument **aParentDocument); \
  nsresult GetChildDocumentCount(uint32_t *aChildDocumentCount); \
  nsresult GetVirtualCursor(nsIAccessiblePivot **aVirtualCursor); \
  nsresult GetChildDocumentAt(uint32_t index, nsIAccessibleDocument **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEDOCUMENT(_to) \
  NS_IMETHOD GetURL(nsAString& aURL) override { return _to GetURL(aURL); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD GetMimeType(nsAString& aMimeType) override { return _to GetMimeType(aMimeType); } \
  NS_IMETHOD GetDocType(nsAString& aDocType) override { return _to GetDocType(aDocType); } \
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) override { return _to GetDOMDocument(aDOMDocument); } \
  NS_IMETHOD GetWindow(mozIDOMWindowProxy **aWindow) override { return _to GetWindow(aWindow); } \
  NS_IMETHOD GetParentDocument(nsIAccessibleDocument **aParentDocument) override { return _to GetParentDocument(aParentDocument); } \
  NS_IMETHOD GetChildDocumentCount(uint32_t *aChildDocumentCount) override { return _to GetChildDocumentCount(aChildDocumentCount); } \
  NS_IMETHOD GetVirtualCursor(nsIAccessiblePivot **aVirtualCursor) override { return _to GetVirtualCursor(aVirtualCursor); } \
  NS_IMETHOD GetChildDocumentAt(uint32_t index, nsIAccessibleDocument **_retval) override { return _to GetChildDocumentAt(index, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEDOCUMENT(_to) \
  NS_IMETHOD GetURL(nsAString& aURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURL(aURL); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD GetMimeType(nsAString& aMimeType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMimeType(aMimeType); } \
  NS_IMETHOD GetDocType(nsAString& aDocType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocType(aDocType); } \
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDOMDocument(aDOMDocument); } \
  NS_IMETHOD GetWindow(mozIDOMWindowProxy **aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindow(aWindow); } \
  NS_IMETHOD GetParentDocument(nsIAccessibleDocument **aParentDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentDocument(aParentDocument); } \
  NS_IMETHOD GetChildDocumentCount(uint32_t *aChildDocumentCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildDocumentCount(aChildDocumentCount); } \
  NS_IMETHOD GetVirtualCursor(nsIAccessiblePivot **aVirtualCursor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVirtualCursor(aVirtualCursor); } \
  NS_IMETHOD GetChildDocumentAt(uint32_t index, nsIAccessibleDocument **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildDocumentAt(index, _retval); } 


#endif /* __gen_nsIAccessibleDocument_h__ */
