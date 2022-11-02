package com.example.bulletinboard.application.usecase;

import com.example.bulletinboard.application.auth.UserAuthRepository;
import com.example.bulletinboard.application.form.UserForm;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import javax.servlet.ServletException;
import javax.servlet.http.HttpServletRequest;

@Service
@RequiredArgsConstructor
public class UserAuthUsecase {
    private final UserAuthRepository authRepository;

    public void userCreate(UserForm userForm, HttpServletRequest request)throws ServletException{
        authRepository.createUser(
                userForm.getUsername(),
                userForm.getPassword()
        );

        request.login(userForm.getUsername(),userForm.getPassword());
    }
}
