// components/QuillEditor.tsx
import dynamic from "next/dynamic";
import React, { useEffect, useRef } from "react";
const ReactQuill = dynamic(() => import("react-quill"), { ssr: false });
import "react-quill/dist/quill.snow.css";

const CustomToolbar = () => (
  <div id="toolbar">
    <select className="ql-font">
      <option value="sans-serif">Sans Serif</option>
      <option value="serif">Serif</option>
      <option value="monospace">Monospace</option>
    </select>
    <select className="ql-size">
      <option value="10px">10</option>
      <option value="12px">12</option>
      <option value="14px">14</option>
      <option value="18px">18</option>
      <option value="24px">24</option>
      <option value="36px">36</option>
    </select>
    <button className="ql-bold" />
    <button className="ql-italic" />
    <button className="ql-underline" />
    <select className="ql-color">
      <option value="red">Red</option>
      <option value="green">Green</option>
      <option value="blue">Blue</option>
      <option value="orange">Orange</option>
      <option value="violet">Violet</option>
      <option value="#d0d1d2">Gray</option>
      <option value="black">Black</option>
    </select>
    <button className="ql-link" />
    <button className="ql-align" value="" />
    <button className="ql-align" value="center" />
    <button className="ql-align" value="right" />
    <button className="ql-list" value="ordered" />
    <button className="ql-list" value="bullet" />
    <button className="ql-indent" value="-1" />
    <button className="ql-indent" value="+1" />
  </div>
);

type Props = {
  value: string;
  onChange: (value: string) => void;
};

const QuillEditor: React.FC<Props> = ({ value, onChange }) => {
  const modules = {
    toolbar: {
      container: "#toolbar",
    },
  };

  return (
    <div>
      <CustomToolbar />
      <ReactQuill value={value} onChange={onChange} modules={modules} />
    </div>
  );
};

export default QuillEditor;
