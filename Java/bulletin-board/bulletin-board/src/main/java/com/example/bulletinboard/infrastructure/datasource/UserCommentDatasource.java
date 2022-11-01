package com.example.bulletinboard.infrastructure.datasource;

import com.example.bulletinboard.application.dto.UserCommentDto;
import com.example.bulletinboard.domain.model.UserComment;
import com.example.bulletinboard.domain.model.UserCommentMapper;
import com.example.bulletinboard.domain.model.UserCommentRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Repository;

@RequiredArgsConstructor
@Repository
public class UserCommentDatasource implements UserCommentRepository {
    private final UserCommentMapper mapper;

    @Override
    public void save(UserComment userComment) {
        mapper.insert(UserCommentDto.from(userComment));
    }
}