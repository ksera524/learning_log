'use client'

import { useState,useRef,useCallback } from "react"

export function ShuffleMemberForm() {
    const [result,setResult] = useState([] as string[]);
    const firstRef = useRef<HTMLInputElement>(null);
    const secondRef = useRef<HTMLInputElement>(null);
    const thirdRef = useRef<HTMLInputElement>(null);

    const callApi = useCallback(async () => {
        const members = [] as string[];
        const refs = [firstRef,secondRef,thirdRef];
        for(const ref of refs) {
            if(ref.current?.value) {
                members.push(ref.current?.value);
            }
        }

        const res = await fetch("/api/shuffle", {
            method:"post",
            body:JSON.stringify({members})
        });

        if (res.ok) {
            const result = await res.json() as {members : string[]};
            setResult(result.members);
        }
    },[])

    return (
        <>
            <label htmlFor="first">1人目:</label>
            <input type="text" ref={firstRef} id="first" name="first" placeholder="1人目の名前を入力" /><br/>
            <label htmlFor="second">2人目:</label>
            <input type="text" ref={secondRef} id="second" name="second" placeholder="2人目の名前を入力" /><br/>
            <label htmlFor="third">3人目:</label>
            <input type="text" ref={thirdRef} id="third" name="third" placeholder="3人目の名前を入力" /><br/>
            <button onClick={callApi}>シャッフル</button>
            <label htmlFor="result">結果</label>
            <output id="result" htmlFor="first second third fourth">{result.join("→")}</output>
        </>
    )
}