use std::io::{Read,Seek};
use crate::binary_reader::BinaryReader;
use std::collections::HashMap;

pub fn get_string_or_default<T:Read+Seek>(pos: u32, reader: &mut BinaryReader<T>) -> String
{
    match pos & 0x80000000 == 0 {
        true => reader.indexed_cstr(pos as u64), // buffer.unpack1("@#{pos}Z*")
        false => {
            let idx = pos & 0x7fffffff;
            match HASHMAP.get(&idx) {
                Some(s) => s.to_string(),
                None => String::from(""),
            }
        }
    }
}

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0 , "AABB");
        m.insert(5 , "AnimationClip");
        m.insert(19 , "AnimationCurve");
        m.insert(34 , "AnimationState");
        m.insert(49 , "Array");
        m.insert(55 , "Base");
        m.insert(60 , "BitField");
        m.insert(69 , "bitset");
        m.insert(76 , "bool");
        m.insert(81 , "char");
        m.insert(86 , "ColorRGBA");
        m.insert(96 , "Component");
        m.insert(106 , "data");
        m.insert(111 , "deque");
        m.insert(117 , "double");
        m.insert(124 , "dynamic_array");
        m.insert(138 , "FastPropertyName");
        m.insert(155 , "first");
        m.insert(161 , "float");
        m.insert(167 , "Font");
        m.insert(172 , "GameObject");
        m.insert(183 , "Generic Mono");
        m.insert(196 , "GradientNEW");
        m.insert(208 , "GUID");
        m.insert(213 , "GUIStyle");
        m.insert(222 , "int");
        m.insert(226 , "list");
        m.insert(231 , "long long");
        m.insert(241 , "map");
        m.insert(245 , "Matrix4x4f");
        m.insert(256 , "MdFour");
        m.insert(263 , "MonoBehaviour");
        m.insert(277 , "MonoScript");
        m.insert(288 , "m_ByteSize");
        m.insert(299 , "m_Curve");
        m.insert(307 , "m_EditorClassIdentifier");
        m.insert(331 , "m_EditorHideFlags");
        m.insert(349 , "m_Enabled");
        m.insert(359 , "m_ExtensionPtr");
        m.insert(374 , "m_GameObject");
        m.insert(387 , "m_Index");
        m.insert(395 , "m_IsArray");
        m.insert(405 , "m_IsStatic");
        m.insert(416 , "m_MetaFlag");
        m.insert(427 , "m_Name");
        m.insert(434 , "m_ObjectHideFlags");
        m.insert(452 , "m_PrefabInternal");
        m.insert(469 , "m_PrefabParentObject");
        m.insert(490 , "m_Script");
        m.insert(499 , "m_StaticEditorFlags");
        m.insert(519 , "m_Type");
        m.insert(526 , "m_Version");
        m.insert(536 , "Object");
        m.insert(543 , "pair");
        m.insert(548 , "PPtr<Component>");
        m.insert(564 , "PPtr<GameObject>");
        m.insert(581 , "PPtr<Material>");
        m.insert(596 , "PPtr<MonoBehaviour>");
        m.insert(616 , "PPtr<MonoScript>");
        m.insert(633 , "PPtr<Object>");
        m.insert(646 , "PPtr<Prefab>");
        m.insert(659 , "PPtr<Sprite>");
        m.insert(672 , "PPtr<TextAsset>");
        m.insert(688 , "PPtr<Texture>");
        m.insert(702 , "PPtr<Texture2D>");
        m.insert(718 , "PPtr<Transform>");
        m.insert(734 , "Prefab");
        m.insert(741 , "Quaternionf");
        m.insert(753 , "Rectf");
        m.insert(759 , "RectInt");
        m.insert(767 , "RectOffset");
        m.insert(778 , "second");
        m.insert(785 , "set");
        m.insert(789 , "short");
        m.insert(795 , "size");
        m.insert(800 , "SInt16");
        m.insert(807 , "SInt32");
        m.insert(814 , "SInt64");
        m.insert(821 , "SInt8");
        m.insert(827 , "staticvector");
        m.insert(840 , "string");
        m.insert(847 , "TextAsset");
        m.insert(857 , "TextMesh");
        m.insert(866 , "Texture");
        m.insert(874 , "Texture2D");
        m.insert(884 , "Transform");
        m.insert(894 , "TypelessData");
        m.insert(907 , "UInt16");
        m.insert(914 , "UInt32");
        m.insert(921 , "UInt64");
        m.insert(928 , "UInt8");
        m.insert(934 , "unsigned int");
        m.insert(947 , "unsigned long long");
        m.insert(966 , "unsigned short");
        m.insert(981 , "vector");
        m.insert(988 , "Vector2f");
        m.insert(997 , "Vector3f");
        m.insert(1006 , "Vector4f");
        m.insert(1015 , "m_ScriptingClassIdentifier");
        m.insert(1042 , "Gradient");
        m.insert(1051 , "Type*");
        m.insert(1057 , "int2_storage");
        m.insert(1070 , "int3_storage");
        m.insert(1083 , "BoundsInt");
        m.insert(1093 , "m_CorrespondingSourceObject");
        m.insert(1121 , "m_PrefabInstance");
        m.insert(1138 , "m_PrefabAsset");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
}

