package com.example.bulletinboard.infrastructure.datasource;

import com.example.bulletinboard.application.dto.UserCommentDto;
import com.example.bulletinboard.application.dto.UserCommentReadDto;
import com.example.bulletinboard.domain.model.UserComment;
import com.example.bulletinboard.domain.model.UserCommentMapper;
import com.example.bulletinboard.domain.model.UserCommentRepository;
import com.example.bulletinboard.domain.model.UserComments;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Repository;

import java.util.List;

@RequiredArgsConstructor
@Repository
public class UserCommentDatasource implements UserCommentRepository {
    private final UserCommentMapper mapper;

    @Override
    public void save(UserComment userComment) {
        mapper.insert(UserCommentDto.from(userComment));
    }

    @Override
    public UserComments select(){
        List<UserCommentReadDto> dtos = mapper.select();
        return UserComments.from(
                dtos.stream().map(dto -> UserComments.UserComment.from(
                        dto.getId(),
                        dto.getName(),
                        dto.getMailAddress(),
                        dto.getComment(),
                        dto.getCreatedAt()
                )).toList()
        );
    }
}