import dynamic from "next/dynamic";
import React, { useEffect } from "react";

const ReactQuill = dynamic(() => import("react-quill"), {
  ssr: false,
  loading: () => <p>Loading...</p>,
});

type DynamicReactQuillProps = {
  content: string;
  setContent: (content: string) => void;
};

const DynamicReactQuill: React.FC<DynamicReactQuillProps> = ({
  content,
  setContent,
}) => {
  // Quill の設定を変更して、<div> タグを使用する
  useEffect(() => {
    if (typeof window !== "undefined") {
      const Quill = require("react-quill").Quill;
      const Block = Quill.import("blots/block");
      Block.tagName = "div";
      Quill.register(Block, true);
    }
  }, []);

  const toolbarOptions = [
    ["bold", "italic", "underline", "strike"], // toggled buttons
    ["blockquote", "code-block"],
    ["link"], // ハイパーリンク機能を追加
    ["image"],
    [{ header: 1 }, { header: 2 }], // custom button values
    [{ list: "ordered" }, { list: "bullet" }],
    [{ script: "sub" }, { script: "super" }], // superscript/subscript
    [{ indent: "-1" }, { indent: "+1" }], // outdent/indent
    [{ direction: "rtl" }], // text direction

    [{ size: ["small", false, "large", "huge"] }], // custom dropdown
    [{ header: [1, 2, 3, 4, 5, 6, false] }],

    [{ color: [] }, { background: [] }], // dropdown with defaults from theme
    [{ font: [] }],
    [{ align: [] }],

    ["clean"], // remove formatting button
  ];

  return (
    <ReactQuill
      theme="snow"
      value={content}
      onChange={(newContent) => {
        setContent(newContent);
      }}
      modules={{ toolbar: toolbarOptions }}
    />
  );
};

export default DynamicReactQuill;
