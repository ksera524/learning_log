package com.example.bulletinboard.application.usecase;


import com.example.bulletinboard.domain.model.UserComment;
import com.example.bulletinboard.domain.model.UserCommentRepository;
import com.example.bulletinboard.presentation.form.CommentForm;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class UserCommentUseCase {
    private final UserCommentRepository repository;
    public void write(CommentForm commentForm){
        UserComment userComment = UserComment.from(
                commentForm.getName(),
                commentForm.getMailAddress(),
                commentForm.getComment()
        );
        repository.save(userComment);
    }
}
