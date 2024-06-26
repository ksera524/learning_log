package com.example.bulletinboard.presentation;

import com.example.bulletinboard.application.usecase.UserCommentUseCase;
import com.example.bulletinboard.domain.model.UserComments;
import com.example.bulletinboard.presentation.form.CommentForm;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Controller;
import org.springframework.validation.BindingResult;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.servlet.ModelAndView;

@RequiredArgsConstructor
@Controller
public class BoardController {
    private final UserCommentUseCase userCommentUseCase;

    @GetMapping("/board")
    public ModelAndView viewBoard(ModelAndView modelAndView){
        UserComments userComments = userCommentUseCase.read();

        modelAndView.setViewName("board");
        modelAndView.addObject("commentForm",new CommentForm());
        modelAndView.addObject("comments",userComments.getValue());
        return modelAndView;
    }

    @PostMapping("/board")
    public ModelAndView postComment(@Validated @ModelAttribute CommentForm comment, BindingResult bindingResult){
        if (bindingResult.hasErrors()){
            ModelAndView modelAndView = new ModelAndView(("/board"));
            modelAndView.addObject("commentForm",comment);
            return modelAndView;
        }
        userCommentUseCase.write(comment);
        return new ModelAndView("redirect:/board");
    }
}
