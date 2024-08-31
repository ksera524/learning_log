import { NextResponse } from "next/server";

export async function POST(request:Request) {
    const {members} = await request.json() as { members: string[]};
    const result = members.sort(() => Math.random() - 0.5);
    return NextResponse.json({ members:result});
}