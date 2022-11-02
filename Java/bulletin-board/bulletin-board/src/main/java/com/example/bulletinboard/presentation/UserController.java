package com.example.bulletinboard.presentation;

import com.example.bulletinboard.application.form.UserForm;
import com.example.bulletinboard.application.usecase.UserAuthUsecase;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Controller;
import org.springframework.validation.BindingResult;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.servlet.ModelAndView;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;

@Slf4j
@Controller
@RequestMapping("/user")
@RequiredArgsConstructor
public class UserController {
    private final UserAuthUsecase userAuthUsecase;


    @GetMapping("/signup")
    public ModelAndView signup(ModelAndView modelAndView) {
        modelAndView.setViewName("user/signup");
        modelAndView.addObject("userForm", new UserForm());

        return modelAndView;
    }

    @PostMapping("/signup")
    public ModelAndView register(
            @Validated @ModelAttribute UserForm userForm,
            BindingResult bindingResult,
            HttpServletRequest request
    ) {
        if(bindingResult.hasErrors()) {
            ModelAndView modelAndView = new ModelAndView("user/signup");
            modelAndView.addObject("userForm", userForm);
            return modelAndView;
        }
        try {
            userAuthUsecase.userCreate(userForm,request);
        } catch (Exception e){
            log.error("ユーザ作成 or ログイン失敗", e);
            return new ModelAndView("redirect:/user");
        }

        return new ModelAndView("redirect:/board");
    }
    @GetMapping
    public ModelAndView loginPage(ModelAndView modelAndView){
        modelAndView.setViewName("user/login");
        modelAndView.addObject("userForm",new UserForm());

        return modelAndView;
    }
}