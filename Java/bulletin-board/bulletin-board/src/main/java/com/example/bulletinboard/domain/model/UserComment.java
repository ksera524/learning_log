package com.example.bulletinboard.domain.model;

import com.example.bulletinboard.domain.type.Comment;
import com.example.bulletinboard.domain.type.MailAddress;
import com.example.bulletinboard.domain.type.Name;
import lombok.AccessLevel;
import lombok.Getter;
import lombok.RequiredArgsConstructor;

import java.util.Random;

@RequiredArgsConstructor(access = AccessLevel.PRIVATE)
@Getter
public class UserComment {
    private final Name name;
    private final MailAddress mailAddress;
    private final Comment comment;

    public static UserComment from(String name,String mailAddress,String comment){
        return new UserComment(
                Name.from(name),
                MailAddress.from(mailAddress),
                Comment.from(comment)
        );
    }

    public Name getName(){
        if (!name.equals("!omikuji")) return name;

        int random = new Random().nextInt(3);
        switch (random){
            case 0 -> {
                return Name.from("大吉");
            }
            case 1 -> {
                return Name.from("中吉");
            }
            default -> {
                return Name.from("小吉");
            }
        }
    }
}
