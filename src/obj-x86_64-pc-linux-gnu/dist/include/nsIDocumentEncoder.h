/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/serializers/nsIDocumentEncoder.idl
 */

#ifndef __gen_nsIDocumentEncoder_h__
#define __gen_nsIDocumentEncoder_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOutputStream; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

class nsRange; /* webidl Range */

namespace mozilla {
namespace dom {
class Selection; /* webidl Selection */
} // namespace dom
} // namespace mozilla

class nsINode;

/* starting interface:    nsIDocumentEncoderNodeFixup */
#define NS_IDOCUMENTENCODERNODEFIXUP_IID_STR "3d9371d8-a2ad-403e-8b0e-8885ad3562e3"

#define NS_IDOCUMENTENCODERNODEFIXUP_IID \
  {0x3d9371d8, 0xa2ad, 0x403e, \
    { 0x8b, 0x0e, 0x88, 0x85, 0xad, 0x35, 0x62, 0xe3 }}

class NS_NO_VTABLE nsIDocumentEncoderNodeFixup : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCUMENTENCODERNODEFIXUP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocumentEncoderNodeFixup;

  /* Node fixupNode (in Node aNode, out boolean aSerializeCloneKids); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FixupNode(nsINode *aNode, bool *aSerializeCloneKids, nsINode **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocumentEncoderNodeFixup, NS_IDOCUMENTENCODERNODEFIXUP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCUMENTENCODERNODEFIXUP \
  NS_IMETHOD FixupNode(nsINode *aNode, bool *aSerializeCloneKids, nsINode **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCUMENTENCODERNODEFIXUP \
  nsresult FixupNode(nsINode *aNode, bool *aSerializeCloneKids, nsINode **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCUMENTENCODERNODEFIXUP(_to) \
  NS_IMETHOD FixupNode(nsINode *aNode, bool *aSerializeCloneKids, nsINode **_retval) override { return _to FixupNode(aNode, aSerializeCloneKids, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCUMENTENCODERNODEFIXUP(_to) \
  NS_IMETHOD FixupNode(nsINode *aNode, bool *aSerializeCloneKids, nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FixupNode(aNode, aSerializeCloneKids, _retval); } 


/* starting interface:    nsIDocumentEncoder */
#define NS_IDOCUMENTENCODER_IID_STR "21f112df-d96f-47da-bfcb-5331273003d1"

#define NS_IDOCUMENTENCODER_IID \
  {0x21f112df, 0xd96f, 0x47da, \
    { 0xbf, 0xcb, 0x53, 0x31, 0x27, 0x30, 0x03, 0xd1 }}

class NS_NO_VTABLE nsIDocumentEncoder : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCUMENTENCODER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocumentEncoder;

  enum {
    OutputSelectionOnly = 1U,
    OutputFormatted = 2U,
    OutputRaw = 4U,
    OutputBodyOnly = 8U,
    OutputPreformatted = 16U,
    OutputWrap = 32U,
    OutputFormatFlowed = 64U,
    OutputAbsoluteLinks = 128U,
    OutputCRLineBreak = 512U,
    OutputLFLineBreak = 1024U,
    OutputNoScriptContent = 2048U,
    OutputNoFramesContent = 4096U,
    OutputNoFormattingInPre = 8192U,
    OutputEncodeBasicEntities = 16384U,
    OutputPersistNBSP = 131072U,
    OutputDontRewriteEncodingDeclaration = 262144U,
    SkipInvisibleContent = 524288U,
    OutputFormatDelSp = 1048576U,
    OutputDropInvisibleBreak = 2097152U,
    OutputIgnoreMozDirty = 4194304U,
    OutputForPlainTextClipboardCopy = 33554432U,
    OutputRubyAnnotation = 67108864U,
    OutputDisallowLineBreaking = 134217728U,
    RequiresReinitAfterOutput = 268435456U
  };

  /* void init (in Document aDocument, in AString aMimeType, in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) = 0;

  /* [noscript] void nativeInit (in Document aDocument, in AString aMimeType, in unsigned long aFlags); */
  NS_IMETHOD NativeInit(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) = 0;

  /* void setSelection (in Selection aSelection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelection(mozilla::dom::Selection *aSelection) = 0;

  /* void setRange (in Range aRange); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRange(nsRange *aRange) = 0;

  /* void setNode (in Node aNode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNode(nsINode *aNode) = 0;

  /* void setContainerNode (in Node aContainer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContainerNode(nsINode *aContainer) = 0;

  /* void setCharset (in ACString aCharset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCharset(const nsACString& aCharset) = 0;

  /* void setWrapColumn (in unsigned long aWrapColumn); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetWrapColumn(uint32_t aWrapColumn) = 0;

  /* readonly attribute AString mimeType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMimeType(nsAString& aMimeType) = 0;

  /* void encodeToStream (in nsIOutputStream aStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncodeToStream(nsIOutputStream *aStream) = 0;

  /* AString encodeToString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncodeToString(nsAString& _retval) = 0;

  /* AString encodeToStringWithContext (out AString aContextString, out AString aInfoString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncodeToStringWithContext(nsAString& aContextString, nsAString& aInfoString, nsAString& _retval) = 0;

  /* AString encodeToStringWithMaxLength (in unsigned long aMaxLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncodeToStringWithMaxLength(uint32_t aMaxLength, nsAString& _retval) = 0;

  /* void setNodeFixup (in nsIDocumentEncoderNodeFixup aFixup); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNodeFixup(nsIDocumentEncoderNodeFixup *aFixup) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocumentEncoder, NS_IDOCUMENTENCODER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCUMENTENCODER \
  NS_IMETHOD Init(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) override; \
  NS_IMETHOD NativeInit(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) override; \
  NS_IMETHOD SetSelection(mozilla::dom::Selection *aSelection) override; \
  NS_IMETHOD SetRange(nsRange *aRange) override; \
  NS_IMETHOD SetNode(nsINode *aNode) override; \
  NS_IMETHOD SetContainerNode(nsINode *aContainer) override; \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override; \
  NS_IMETHOD SetWrapColumn(uint32_t aWrapColumn) override; \
  NS_IMETHOD GetMimeType(nsAString& aMimeType) override; \
  NS_IMETHOD EncodeToStream(nsIOutputStream *aStream) override; \
  NS_IMETHOD EncodeToString(nsAString& _retval) override; \
  NS_IMETHOD EncodeToStringWithContext(nsAString& aContextString, nsAString& aInfoString, nsAString& _retval) override; \
  NS_IMETHOD EncodeToStringWithMaxLength(uint32_t aMaxLength, nsAString& _retval) override; \
  NS_IMETHOD SetNodeFixup(nsIDocumentEncoderNodeFixup *aFixup) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCUMENTENCODER \
  nsresult Init(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags); \
  nsresult NativeInit(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags); \
  nsresult SetSelection(mozilla::dom::Selection *aSelection); \
  nsresult SetRange(nsRange *aRange); \
  nsresult SetNode(nsINode *aNode); \
  nsresult SetContainerNode(nsINode *aContainer); \
  nsresult SetCharset(const nsACString& aCharset); \
  nsresult SetWrapColumn(uint32_t aWrapColumn); \
  nsresult GetMimeType(nsAString& aMimeType); \
  nsresult EncodeToStream(nsIOutputStream *aStream); \
  nsresult EncodeToString(nsAString& _retval); \
  nsresult EncodeToStringWithContext(nsAString& aContextString, nsAString& aInfoString, nsAString& _retval); \
  nsresult EncodeToStringWithMaxLength(uint32_t aMaxLength, nsAString& _retval); \
  nsresult SetNodeFixup(nsIDocumentEncoderNodeFixup *aFixup); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCUMENTENCODER(_to) \
  NS_IMETHOD Init(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) override { return _to Init(aDocument, aMimeType, aFlags); } \
  NS_IMETHOD NativeInit(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) override { return _to NativeInit(aDocument, aMimeType, aFlags); } \
  NS_IMETHOD SetSelection(mozilla::dom::Selection *aSelection) override { return _to SetSelection(aSelection); } \
  NS_IMETHOD SetRange(nsRange *aRange) override { return _to SetRange(aRange); } \
  NS_IMETHOD SetNode(nsINode *aNode) override { return _to SetNode(aNode); } \
  NS_IMETHOD SetContainerNode(nsINode *aContainer) override { return _to SetContainerNode(aContainer); } \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override { return _to SetCharset(aCharset); } \
  NS_IMETHOD SetWrapColumn(uint32_t aWrapColumn) override { return _to SetWrapColumn(aWrapColumn); } \
  NS_IMETHOD GetMimeType(nsAString& aMimeType) override { return _to GetMimeType(aMimeType); } \
  NS_IMETHOD EncodeToStream(nsIOutputStream *aStream) override { return _to EncodeToStream(aStream); } \
  NS_IMETHOD EncodeToString(nsAString& _retval) override { return _to EncodeToString(_retval); } \
  NS_IMETHOD EncodeToStringWithContext(nsAString& aContextString, nsAString& aInfoString, nsAString& _retval) override { return _to EncodeToStringWithContext(aContextString, aInfoString, _retval); } \
  NS_IMETHOD EncodeToStringWithMaxLength(uint32_t aMaxLength, nsAString& _retval) override { return _to EncodeToStringWithMaxLength(aMaxLength, _retval); } \
  NS_IMETHOD SetNodeFixup(nsIDocumentEncoderNodeFixup *aFixup) override { return _to SetNodeFixup(aFixup); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCUMENTENCODER(_to) \
  NS_IMETHOD Init(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aDocument, aMimeType, aFlags); } \
  NS_IMETHOD NativeInit(mozilla::dom::Document *aDocument, const nsAString& aMimeType, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NativeInit(aDocument, aMimeType, aFlags); } \
  NS_IMETHOD SetSelection(mozilla::dom::Selection *aSelection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelection(aSelection); } \
  NS_IMETHOD SetRange(nsRange *aRange) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRange(aRange); } \
  NS_IMETHOD SetNode(nsINode *aNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNode(aNode); } \
  NS_IMETHOD SetContainerNode(nsINode *aContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContainerNode(aContainer); } \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCharset(aCharset); } \
  NS_IMETHOD SetWrapColumn(uint32_t aWrapColumn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWrapColumn(aWrapColumn); } \
  NS_IMETHOD GetMimeType(nsAString& aMimeType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMimeType(aMimeType); } \
  NS_IMETHOD EncodeToStream(nsIOutputStream *aStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeToStream(aStream); } \
  NS_IMETHOD EncodeToString(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeToString(_retval); } \
  NS_IMETHOD EncodeToStringWithContext(nsAString& aContextString, nsAString& aInfoString, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeToStringWithContext(aContextString, aInfoString, _retval); } \
  NS_IMETHOD EncodeToStringWithMaxLength(uint32_t aMaxLength, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeToStringWithMaxLength(aMaxLength, _retval); } \
  NS_IMETHOD SetNodeFixup(nsIDocumentEncoderNodeFixup *aFixup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNodeFixup(aFixup); } 

template<class T> struct already_AddRefed;
bool
do_getDocumentTypeSupportedForEncoding(const char* aContentType);
already_AddRefed<nsIDocumentEncoder>
do_createDocumentEncoder(const char* aContentType);
already_AddRefed<nsIDocumentEncoder>
do_createHTMLCopyEncoder();

#endif /* __gen_nsIDocumentEncoder_h__ */
