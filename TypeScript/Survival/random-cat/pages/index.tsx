import { NextPage, GetServerSideProps } from "next";
import { useState } from "react";
import styles from "./index.module.css";

type Props = {
  initialImageURL: string;
};

const IndexPage: NextPage<Props> = ({ initialImageURL }) => {
  const [imageUrl, setImageUrl] = useState(initialImageURL);
  const [loading, setLoading] = useState(false);

  const handleClick = async () => {
    setLoading(true);
    const image = await fetchImage();
    setImageUrl(image.url);
    setLoading(false);
  };

  return (
    <div className={styles.page}>
      <button onClick={handleClick} className={styles.button}>
        Change Cat
      </button>
      <div className={styles.frame}>
        {loading || <img src={imageUrl} className={styles.img} />}
      </div>
    </div>
  );
};

export const getServerSideProps: GetServerSideProps<Props> = async () => {
  const image = await fetchImage();
  return {
    props: {
      initialImageURL: image.url,
    },
  };
};

type Image = {
  url: string;
};

const fetchImage = async (): Promise<Image> => {
  const res = await fetch("https://api.thecatapi.com/v1/images/search");
  const data = await res.json();
  console.log(data);
  return data[0];
};

export default IndexPage;
