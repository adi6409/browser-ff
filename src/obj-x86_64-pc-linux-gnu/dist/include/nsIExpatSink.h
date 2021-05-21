/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/parser/htmlparser/nsIExpatSink.idl
 */

#ifndef __gen_nsIExpatSink_h__
#define __gen_nsIExpatSink_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIScriptError; /* forward declaration */


/* starting interface:    nsIExpatSink */
#define NS_IEXPATSINK_IID_STR "01f681af-0f22-4725-a914-0d396114daf0"

#define NS_IEXPATSINK_IID \
  {0x01f681af, 0x0f22, 0x4725, \
    { 0xa9, 0x14, 0x0d, 0x39, 0x61, 0x14, 0xda, 0xf0 }}

class NS_NO_VTABLE nsIExpatSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXPATSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIExpatSink;

  /* void HandleStartElement (in wstring aName, [array, size_is (aAttsCount)] in wstring aAtts, in unsigned long aAttsCount, in unsigned long aLineNumber, in unsigned long aColumnNumber); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleStartElement(const char16_t * aName, const char16_t * *aAtts, uint32_t aAttsCount, uint32_t aLineNumber, uint32_t aColumnNumber) = 0;

  /* void HandleEndElement (in wstring aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEndElement(const char16_t * aName) = 0;

  /* void HandleComment (in wstring aCommentText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleComment(const char16_t * aCommentText) = 0;

  /* void HandleCDataSection ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleCDataSection(const char16_t * aData, uint32_t aLength) = 0;

  /* void HandleDoctypeDecl (in AString aSubset, in AString aName, in AString aSystemId, in AString aPublicId, in nsISupports aCatalogData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleDoctypeDecl(const nsAString& aSubset, const nsAString& aName, const nsAString& aSystemId, const nsAString& aPublicId, nsISupports *aCatalogData) = 0;

  /* void HandleCharacterData ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleCharacterData(const char16_t * aData, uint32_t aLength) = 0;

  /* void HandleProcessingInstruction (in wstring aTarget, in wstring aData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleProcessingInstruction(const char16_t * aTarget, const char16_t * aData) = 0;

  /* void HandleXMLDeclaration (in wstring aVersion, in wstring aEncoding, in long aStandalone); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleXMLDeclaration(const char16_t * aVersion, const char16_t * aEncoding, int32_t aStandalone) = 0;

  /* boolean ReportError (in wstring aErrorText, in wstring aSourceText, in nsIScriptError aError); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReportError(const char16_t * aErrorText, const char16_t * aSourceText, nsIScriptError *aError, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIExpatSink, NS_IEXPATSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXPATSINK \
  NS_IMETHOD HandleStartElement(const char16_t * aName, const char16_t * *aAtts, uint32_t aAttsCount, uint32_t aLineNumber, uint32_t aColumnNumber) override; \
  NS_IMETHOD HandleEndElement(const char16_t * aName) override; \
  NS_IMETHOD HandleComment(const char16_t * aCommentText) override; \
  NS_IMETHOD HandleCDataSection(const char16_t * aData, uint32_t aLength) override; \
  NS_IMETHOD HandleDoctypeDecl(const nsAString& aSubset, const nsAString& aName, const nsAString& aSystemId, const nsAString& aPublicId, nsISupports *aCatalogData) override; \
  NS_IMETHOD HandleCharacterData(const char16_t * aData, uint32_t aLength) override; \
  NS_IMETHOD HandleProcessingInstruction(const char16_t * aTarget, const char16_t * aData) override; \
  NS_IMETHOD HandleXMLDeclaration(const char16_t * aVersion, const char16_t * aEncoding, int32_t aStandalone) override; \
  NS_IMETHOD ReportError(const char16_t * aErrorText, const char16_t * aSourceText, nsIScriptError *aError, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXPATSINK \
  nsresult HandleStartElement(const char16_t * aName, const char16_t * *aAtts, uint32_t aAttsCount, uint32_t aLineNumber, uint32_t aColumnNumber); \
  nsresult HandleEndElement(const char16_t * aName); \
  nsresult HandleComment(const char16_t * aCommentText); \
  nsresult HandleCDataSection(const char16_t * aData, uint32_t aLength); \
  nsresult HandleDoctypeDecl(const nsAString& aSubset, const nsAString& aName, const nsAString& aSystemId, const nsAString& aPublicId, nsISupports *aCatalogData); \
  nsresult HandleCharacterData(const char16_t * aData, uint32_t aLength); \
  nsresult HandleProcessingInstruction(const char16_t * aTarget, const char16_t * aData); \
  nsresult HandleXMLDeclaration(const char16_t * aVersion, const char16_t * aEncoding, int32_t aStandalone); \
  nsresult ReportError(const char16_t * aErrorText, const char16_t * aSourceText, nsIScriptError *aError, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXPATSINK(_to) \
  NS_IMETHOD HandleStartElement(const char16_t * aName, const char16_t * *aAtts, uint32_t aAttsCount, uint32_t aLineNumber, uint32_t aColumnNumber) override { return _to HandleStartElement(aName, aAtts, aAttsCount, aLineNumber, aColumnNumber); } \
  NS_IMETHOD HandleEndElement(const char16_t * aName) override { return _to HandleEndElement(aName); } \
  NS_IMETHOD HandleComment(const char16_t * aCommentText) override { return _to HandleComment(aCommentText); } \
  NS_IMETHOD HandleCDataSection(const char16_t * aData, uint32_t aLength) override { return _to HandleCDataSection(aData, aLength); } \
  NS_IMETHOD HandleDoctypeDecl(const nsAString& aSubset, const nsAString& aName, const nsAString& aSystemId, const nsAString& aPublicId, nsISupports *aCatalogData) override { return _to HandleDoctypeDecl(aSubset, aName, aSystemId, aPublicId, aCatalogData); } \
  NS_IMETHOD HandleCharacterData(const char16_t * aData, uint32_t aLength) override { return _to HandleCharacterData(aData, aLength); } \
  NS_IMETHOD HandleProcessingInstruction(const char16_t * aTarget, const char16_t * aData) override { return _to HandleProcessingInstruction(aTarget, aData); } \
  NS_IMETHOD HandleXMLDeclaration(const char16_t * aVersion, const char16_t * aEncoding, int32_t aStandalone) override { return _to HandleXMLDeclaration(aVersion, aEncoding, aStandalone); } \
  NS_IMETHOD ReportError(const char16_t * aErrorText, const char16_t * aSourceText, nsIScriptError *aError, bool *_retval) override { return _to ReportError(aErrorText, aSourceText, aError, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXPATSINK(_to) \
  NS_IMETHOD HandleStartElement(const char16_t * aName, const char16_t * *aAtts, uint32_t aAttsCount, uint32_t aLineNumber, uint32_t aColumnNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleStartElement(aName, aAtts, aAttsCount, aLineNumber, aColumnNumber); } \
  NS_IMETHOD HandleEndElement(const char16_t * aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEndElement(aName); } \
  NS_IMETHOD HandleComment(const char16_t * aCommentText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleComment(aCommentText); } \
  NS_IMETHOD HandleCDataSection(const char16_t * aData, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleCDataSection(aData, aLength); } \
  NS_IMETHOD HandleDoctypeDecl(const nsAString& aSubset, const nsAString& aName, const nsAString& aSystemId, const nsAString& aPublicId, nsISupports *aCatalogData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleDoctypeDecl(aSubset, aName, aSystemId, aPublicId, aCatalogData); } \
  NS_IMETHOD HandleCharacterData(const char16_t * aData, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleCharacterData(aData, aLength); } \
  NS_IMETHOD HandleProcessingInstruction(const char16_t * aTarget, const char16_t * aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleProcessingInstruction(aTarget, aData); } \
  NS_IMETHOD HandleXMLDeclaration(const char16_t * aVersion, const char16_t * aEncoding, int32_t aStandalone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleXMLDeclaration(aVersion, aEncoding, aStandalone); } \
  NS_IMETHOD ReportError(const char16_t * aErrorText, const char16_t * aSourceText, nsIScriptError *aError, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReportError(aErrorText, aSourceText, aError, _retval); } 


#endif /* __gen_nsIExpatSink_h__ */
